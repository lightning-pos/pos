/**
 * Utility functions for handling number formatting and input validation
 */

/**
 * Sanitizes a decimal input value to ensure it follows the specified format
 * 
 * @param value - The input value to sanitize
 * @param maxDecimalPlaces - Maximum number of decimal places allowed (default: 4)
 * @param allowNegative - Whether to allow negative values (default: false)
 * @returns The sanitized decimal value as a string
 */
export const sanitizeDecimalInput = (
  value: string,
  maxDecimalPlaces: number = 4,
  allowNegative: boolean = false
): string => {
  // Create regex pattern based on whether negative values are allowed
  const regexPattern = allowNegative ? /[^0-9.-]/g : /[^0-9.]/g;
  
  // Remove disallowed characters
  let sanitizedValue = value.replace(regexPattern, '');
  
  // Handle negative sign if allowed
  if (allowNegative && sanitizedValue.includes('-')) {
    // Ensure negative sign is only at the beginning
    const hasNegativeSign = sanitizedValue.startsWith('-');
    sanitizedValue = sanitizedValue.replace(/-/g, '');
    if (hasNegativeSign) {
      sanitizedValue = '-' + sanitizedValue;
    }
  }
  
  // Handle decimal points
  const parts = sanitizedValue.split('.');
  
  // If there are multiple decimal points, keep only the first one
  if (parts.length > 2) {
    sanitizedValue = parts[0] + '.' + parts[1];
  }
  
  // Limit decimal places
  if (parts.length === 2 && parts[1].length > maxDecimalPlaces) {
    sanitizedValue = parts[0] + '.' + parts[1].slice(0, maxDecimalPlaces);
  }
  
  return sanitizedValue;
};

/**
 * Formats a number as currency
 * 
 * @param value - The number to format
 * @param currency - The currency code (default: 'INR')
 * @returns Formatted currency string
 */
export const formatCurrency = (
  value: number | string,
  currency: string = 'INR'
): string => {
  const numValue = typeof value === 'string' ? parseFloat(value) : value;
  
  return new Intl.NumberFormat('en-IN', {
    style: 'currency',
    currency
  }).format(numValue);
};
