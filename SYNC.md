# Offline-First Sync Implementation Guide

This document outlines the implementation of offline-first capabilities with cloud synchronization for the Lightning POS system, including support for multi-shop deployments.

## Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [Implementation Details](#implementation-details)
3. [Multi-Shop Synchronization](#multi-shop-synchronization)
4. [Data Synchronization Service](#data-synchronization-service)
5. [Schema Migration Strategy](#schema-migration-strategy)
6. [Conflict Resolution](#conflict-resolution)
7. [Security Considerations](#security-considerations)
8. [Implementation Timeline](#implementation-timeline)
9. [Future Considerations](#future-considerations)

## Architecture Overview

The system uses a hybrid architecture with a central hub:

- **Local SQLite Database**: Each POS instance has its own local SQLite database
- **Cloud SQLite Database**: Each POS instance syncs with its cloud SQLite database
- **Central Hub Database**: Consolidates data from all shops and serves as the primary source of truth
- **ETL Sync Service**: Manages data flow between shop cloud databases and the central hub

### Data Flow

1. **Catalog Data Flow**:
   - Central Database → ETL Service → Cloud SQLite → POS Instance
   - POS Instance → Cloud SQLite → ETL Service → Central Database → ETL Service → Other Cloud SQLites → Other POS Instances

2. **Sales Data Flow**:
   - POS Instance → Cloud SQLite → ETL Service → Central Database (for aggregation)
   - Sales data does NOT sync between different shops

### Technology Stack

- **Database**: SQLite (local) and Turso (cloud)
- **Sync Mechanism**: Turso's offline writes feature using SQLite's WAL (Write-Ahead Log)
- **ETL Service**: Rust-based service for cross-database synchronization
- **Backend**: Rust with Tauri
- **Frontend**: Next.js

## Implementation Details

### Backend Integration (Rust/Tauri)

#### Backend Components

```
# Dependencies
Add libsql with sync feature to Cargo.toml

# AppService Structure
AppService {
    db: Database,                    // Turso/libSQL database
    conn: Connection,                // Database connection
    state: SessionState,             // Current session state
    shop_id: String,                 // Current shop identifier
    hub_sync_config: SyncConfig      // Sync configuration
}

# Initialize AppService
function initialize_app_service(db_path, shop_id, sync_url, auth_token, is_hub):
    if sync_url and auth_token exist:
        configure database with sync options
    else:
        open database without sync

    return AppService instance

# Sync with Schema Check
function sync_with_schema_check():
    local_version = get local schema version
    central_version = get central schema version

    if local_version < central_version:
        return error "Schema version mismatch, update required"

    perform sync
    return success
```

#### Command Implementation

```
# Create Sales Order with Shop ID
function create_sales_order(order_data):
    begin transaction
        create new order with shop_id
        insert order into database
        update sync metadata with pending status
    commit transaction
    return order
```

### Frontend Integration (Next.js)

#### Frontend Components

```
# Sync Status Component
function SyncStatus():
    state:
        syncStatus: 'online' | 'offline' | 'syncing'
        lastSynced: timestamp or null
        needsUpdate: boolean
        errorMessage: string

    # Monitor online status
    listen for online/offline events

    # Manual sync function
    function triggerSync():
        set status to syncing
        try:
            invoke backend sync
            update lastSynced timestamp
            show success notification
        catch error:
            if error is schema mismatch:
                show update required message
            else:
                show error notification

    # Auto-sync when coming online
    when syncStatus changes to online:
        trigger sync

    # Render appropriate UI based on state
    render sync status indicator, last sync time, and sync button
```

#### Background Sync Service

```
# Background Sync Service
function startSyncService():
    set sync interval to 5 minutes

    perform initial sync

    set up periodic sync timer

    add focus event listener to sync when app regains focus

    return cleanup function

function performSync():
    if online:
        try to sync with database
```

## Multi-Shop Synchronization

### Database Schema Modifications

```
# Add shop identification to tables
1. Add shop_id to shop-specific tables:
   - sales_orders
   - sales_order_items
   - sales_order_payments
   - expenses
   - etc.

2. Add tracking columns to shared catalog tables:
   - origin_shop_id: shop that created the record
   - last_modified_shop_id: shop that last modified the record
   - last_modified_at: timestamp of last modification

3. Create sync metadata table:
   - table_name: which table the record belongs to
   - record_id: unique identifier of the record
   - shop_id: shop that owns/modified the record
   - sync_status: pending, synced, or conflict

4. Create shop configuration table:
   - id: unique shop identifier
   - name: shop name
   - sync_url: URL for cloud sync
   - is_active: whether shop is active
```

### Shop-Specific Queries

```
# Get Sales Orders for Current Shop
function get_sales_orders(limit, offset):
    query sales_orders table
    filter by current shop_id
    order by created_at descending
    apply limit and offset
    return filtered orders
```

### Initial Shop Setup

```
# Provision New Shop
function provision_new_shop(shop_name, hub_url, auth_token):
    generate unique shop_id
    create new database file
    initialize schema
    insert shop record
    perform initial sync to get catalog data
    return shop_id
```

## Data Synchronization Service

A dedicated ETL (Extract, Transform, Load) service manages the data flow between shop cloud databases and the central hub database. This service is critical for maintaining data consistency across the entire system while respecting different sync rules for different data types.

### ETL Service Components

```
# ETL Service Architecture
1. Scheduler: Triggers sync jobs based on schedule or events
2. Extractor: Connects to databases and extracts changed data
3. Transformer: Applies business rules to the data
4. Loader: Writes transformed data to target database
5. Conflict Resolver: Handles data conflicts
6. Monitor: Tracks sync status and performance
```

### Sync Process Flow

```
# Shop to Central Hub Sync
function sync_shop_to_hub(shop_id):
    # Extract changes from shop's cloud database
    changes = extract_changes(shop_id, last_sync_time)

    for each record in changes:
        # Apply transformation rules based on data type
        if record.table is catalog_table:
            # Apply catalog-specific transformations
            transformed = transform_catalog_data(record)
            # Check for conflicts
            if conflict_exists(transformed):
                resolved = resolve_catalog_conflict(transformed, existing_record, is_hub=true)
                load_to_hub(resolved)
                mark_for_propagation(resolved)  # Will be synced to other shops
            else:
                load_to_hub(transformed)
                mark_for_propagation(transformed)
        else:  # Sales or shop-specific data
            # Apply shop-specific transformations
            transformed = transform_shop_data(record)
            load_to_hub(transformed)

    update_last_sync_time(shop_id)

# Central Hub to Shop Sync (for catalog data)
function sync_hub_to_shop(shop_id):
    # Get catalog changes that need propagation to this shop
    catalog_changes = get_pending_propagations(shop_id)

    for each record in catalog_changes:
        # Skip if this shop originated the change
        if record.origin_shop_id == shop_id:
            continue

        # Transform for shop consumption
        transformed = transform_for_shop(record)

        # Load to shop's cloud database
        load_to_shop_cloud(shop_id, transformed)

        # Mark as propagated to this shop
        mark_propagated(record.id, shop_id)
```

### Implementation Options

```
# ETL Service Implementation Options
1. Scheduled Service:
   - Runs on fixed intervals
   - Processes all pending changes
   - Good for predictable workloads

2. Event-Driven Service:
   - Triggered by database change events
   - Processes changes in near real-time
   - More responsive but more complex

3. Hybrid Approach:
   - Event notifications trigger sync jobs
   - Scheduled jobs catch any missed events
   - Combines responsiveness with reliability
```

## Schema Migration Strategy

We use an enforced schema update approach where clients must update their schema before syncing if there's a version mismatch.

### Schema Version Tracking

```
# Schema Version Table
CREATE TABLE schema_version (
    version INTEGER NOT NULL,
    updated_at TIMESTAMP NOT NULL
)
```

### Version Check Before Sync

```
# Check Schema Version Before Sync
function sync_with_schema_check():
    local_version = get local schema version
    central_version = get central schema version

    if local_version < central_version:
        return error "Update required before sync"

    # Versions match, proceed with sync
    perform sync
    return success

function get_schema_version():
    ensure schema_version table exists
    query for highest version number
    return version or 0 if none found
```

### Schema Migration on Update

```
# Apply Schema Migrations
function apply_schema_migrations():
    current_version = get current schema version
    target_version = version for this application release

    if current_version < target_version:
        begin transaction
            # Apply migrations sequentially
            if current_version < 1:
                add shop_id to sales_orders

            if current_version < 2:
                add tracking columns to items

            # Update schema version
            update schema_version table
        commit transaction

        log "Schema updated from version X to Y"
```

## Conflict Resolution

### Conflict Resolution Strategy

```
# Resolve Catalog Conflicts
function resolve_catalog_conflict(local_item, remote_item, is_hub):
    if is_hub:
        # Hub always wins in conflicts
        return remote_item
    else:
        # For shop instances, use timestamp-based resolution
        if local_item.last_modified_at > remote_item.last_modified_at:
            return local_item
        else:
            return remote_item
```

### Sync with Conflict Resolution

```
# Sync with Conflict Resolution
function sync_with_conflict_resolution():
    try:
        perform sync
        return success
    catch SyncConflict:
        log "Conflict detected, attempting to resolve"

        # Try to resolve with rebase strategy
        set sync options with rebase strategy
        retry sync with new options
        return success
    catch other errors:
        return error
```

## Security Considerations

### Access Control

```
# Validate Shop Access
function validate_shop_access(service, record_shop_id):
    # Only allow access if:
    # 1. Record belongs to current shop, or
    # 2. Current instance is the hub
    if service.shop_id != record_shop_id and not service.is_hub:
        return AccessDeniedError
    return success
```

### Secure Sync Configuration

Store sync credentials securely and use HTTPS for all sync communications.

```
# Get Secure Sync Credentials
function get_sync_credentials():
    # Use OS keychain or secure storage
    url = keychain.get_password("app_name", "sync_url")
    token = keychain.get_password("app_name", "sync_token")
    return (url, token)
```

## Implementation Timeline

1. **Phase 1: Basic Offline-First (2-3 weeks)**
   - Set up Turso account and database
   - Integrate libSQL client
   - Implement basic sync functionality
   - Add sync status UI components

2. **Phase 2: Multi-Shop Support (2-3 weeks)**
   - Modify schema to include shop_id
   - Implement shop-specific filtering
   - Set up central hub database
   - Configure bidirectional sync with different rules

3. **Phase 3: ETL Sync Service (2-3 weeks)**
   - Develop core ETL service components
   - Implement shop-to-hub sync logic
   - Implement hub-to-shop sync logic
   - Set up monitoring and error handling

4. **Phase 4: Schema Migration (1-2 weeks)**
   - Implement schema version tracking
   - Add version check before sync
   - Create update notification UI
   - Test migration scenarios

5. **Phase 5: Testing and Optimization (2-3 weeks)**
   - Test with various network conditions
   - Test multi-shop scenarios
   - Test conflict resolution
   - Optimize sync performance

## Future Considerations

The following sections outline important aspects to consider as the implementation progresses. These topics should be addressed in future iterations of this document.

### Disaster Recovery and Data Integrity

- Backup strategies for all database types (local, cloud, central hub)
- Recovery procedures for sync failures or database corruption
- Data validation mechanisms to ensure integrity across systems
- Point-in-time recovery options
- Consistency checks between different database instances

### Network Considerations

- Bandwidth requirements and optimization strategies
- Handling poor network conditions beyond basic offline functionality
- Sync throttling or rate limiting for large datasets
- Compression strategies for data transfer
- Prioritization of critical data during limited connectivity

### Monitoring and Observability

- Specific metrics to track for sync health and performance
- Logging strategies for debugging sync issues
- Alerting mechanisms for sync failures or anomalies
- Dashboard for sync status across all shops
- Historical sync performance analytics

### Initial Data Migration

- Handling initial data migration for existing shops
- Strategies for bootstrapping the system with existing data
- Data cleansing and normalization during migration
- Verification procedures after initial migration
- Rollback strategies for failed migrations

### User Experience During Sync

- UI/UX considerations during sync operations
- Progress indicators and estimated time remaining for large syncs
- User notifications for sync events
- Handling user interactions during sync operations
- Error messaging and recovery options for users

### Testing Strategy

- Comprehensive testing approach for sync functionality
- Specific test scenarios and test data generation strategies
- Automated testing approaches for sync components
- Chaos testing for network and service failures
- Performance testing under various load conditions

### Deployment Strategy

- How to deploy the ETL service
- Infrastructure requirements
- CI/CD approaches for the sync components
- Blue/green deployment strategies
- Rollback procedures for failed deployments

### Versioning and Compatibility

- API versioning between components
- Backward compatibility considerations
- Feature flags for gradual rollout
- Deprecation policies for old versions
- Migration paths for clients on different versions

### Performance Benchmarks

- Expected performance metrics and benchmarks
- Optimization strategies for large datasets
- Scaling considerations for growing number of shops
- Database indexing strategies for sync operations
- Caching strategies for frequently accessed data

### Maintenance and Operations

- Routine maintenance tasks
- Operational procedures for the sync infrastructure
- Troubleshooting guide for common sync issues
- On-call procedures for critical failures
- Documentation and knowledge transfer for operations team

### Distributed Simulation Testing

- End-to-end simulation environment mimicking multiple shops and central hub
- Automated test scenarios that simulate real-world usage patterns
- Network condition simulators (latency, packet loss, disconnections)
- Time-based simulations to test long-running operations and edge cases
- Load testing with simulated data volumes matching production scale
- Chaos engineering approaches to test resilience to component failures
- Clock skew simulations to test timestamp-based conflict resolution
- Multi-region deployment simulations to test geographic distribution
- Concurrent operation simulations to identify race conditions
- Long-running simulations to identify memory leaks and resource exhaustion
