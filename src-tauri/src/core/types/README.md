# Core Type Implementation Guide

## Purpose
Types in this directory enforce domain logic while integrating with:
- Database (Diesel ORM)
- GraphQL API
- Business rule validation

## Implementation Checklist
1. Derive core traits and Diesel attributes:
   ```rust
   #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)] // Core traits
   #[derive(diesel::AsExpression)] // Diesel query integration
   ```
2. Add ORM attributes:
   ```rust
   #[diesel(sql_type = Text)]  // Or appropriate SQL type
   ```
3. GraphQL integration:
   ```rust
   #[graphql_scalar]
   struct MyType;
   ```
4. GraphQL integration (non-transparent):
   ```rust
   #[graphql_scalar]
   impl GraphQLScalar for Money {
       fn parse_token(value: ScalarToken<'_>) -> ParseScalarResult<Self> {
           /* parsing logic */
       }

       fn to_output(&self) -> Value {
           /* serialization logic */
       }

       fn from_input(value: &InputValue) -> Result<Self, String> {
           /* deserialization logic */
       }
   }
   ```
5. Implement database traits:
   - `Queryable` for deserialization
   - `ToSql` for serialization

## Macro Explanations
| Macro/Attribute                   | Purpose                                                          |
| --------------------------------- | ---------------------------------------------------------------- |
| `#[derive(diesel::AsExpression)]` | Diesel: Enables use in queries                                   |
| `#[diesel(sql_type)]`             | Maps Rust type to SQL column type                                |
| `#[graphql_scalar]`               | Exposes type as GraphQL scalar                                   |
| `#[graphql(transparent)]`         | Delegates scalar representation                                  |
| `#[graphql_scalar]`               | Requires manual parse_token/to_output/from_input implementations |

## Testing Practices
1. Unit test all operator overloads
2. Verify database roundtrips (serialize/deserialize)
3. Test GraphQL input/output conversions
4. Include edge cases (min/max values, null boundaries)
