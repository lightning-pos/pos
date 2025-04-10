/**
 * Date formatting utilities for the application
 *
 * These functions help to standardize date formatting across the application,
 * particularly for GraphQL API interactions where specific formats are required.
 */

/**
 * Formats a date to 'YYYY-MM-DD' format
 * @param date Date object or date string
 * @returns Formatted date string in 'YYYY-MM-DD' format
 */
export const formatDateYMD = (date: Date | string | null | undefined): string => {
    if (!date) return ''

    const dateObj = date instanceof Date ? date : new Date(date)
    if (isNaN(dateObj.getTime())) return ''

    return dateObj.toISOString().split('T')[0]
}

/**
 * Formats a date to LocalDateTime format (YYYY-MM-DD HH:MM:SS) expected by the GraphQL API
 *
 * This format is required for LocalDateTime scalar type in the GraphQL schema.
 * It removes timezone information and milliseconds from the ISO string.
 *
 * @param date Date object or date string
 * @returns Formatted date string in 'YYYY-MM-DD HH:MM:SS' format
 */
export const formatToLocalDateTime = (date: Date | string | null | undefined): string => {
    if (!date) {
        date = new Date()
    }

    const dateObj = date instanceof Date ? date : new Date(date)
    if (isNaN(dateObj.getTime())) {
        throw new Error('Invalid date provided')
    }

    // Format: YYYY-MM-DD HH:MM:SS
    return (
        dateObj.getUTCFullYear() + '-' +
        String(dateObj.getUTCMonth() + 1).padStart(2, '0') + '-' +
        String(dateObj.getUTCDate()).padStart(2, '0') + ' ' +
        String(dateObj.getUTCHours()).padStart(2, '0') + ':' +
        String(dateObj.getUTCMinutes()).padStart(2, '0') + ':' +
        String(dateObj.getUTCSeconds()).padStart(2, '0')
    )
}

/**
 * Formats a date to display format (localized)
 * @param date Date object or date string
 * @returns Formatted date string in locale-specific format
 */
export const formatDateForDisplay = (date: Date | string | null | undefined): string => {
    if (!date) return ''

    const dateObj = date instanceof Date ? date : new Date(date)
    if (isNaN(dateObj.getTime())) return ''

    return dateObj.toLocaleDateString()
}
