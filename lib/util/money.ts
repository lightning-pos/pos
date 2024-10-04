/**
 * Represents a monetary amount in a specific currency.
 * Usage:
 * const m = money(10000, 'USD'); // $100.00
 * console.log(m.format()); // $100.00
 */
interface Money {
  amount: number;
  currency: string;
  add(other: Money): Money;
  subtract(other: Money): Money;
  multiply(multiplier: number): Money;
  divide(divisor: number): Money;
  format(locale?: string): string;
  toDecimal(): number;
  toBaseUnits(): number;
}

/**
 * Configuration for a currency.
 */
interface CurrencyConfig {
  code: string;
  decimalPlaces: number;
}

/**
 * Currency configurations.
 */
const currencyConfigs: { [key: string]: CurrencyConfig } = {
  INR: { code: 'INR', decimalPlaces: 2 },
  USD: { code: 'USD', decimalPlaces: 2 },
  EUR: { code: 'EUR', decimalPlaces: 2 },
  // Add more currencies as needed
};

/**
 * Gets the currency configuration for a given currency code.
 *
 * @param currencyCode - The ISO 4217 currency code
 * @returns The currency configuration
 * @throws Error if the currency is not supported
 */
const getCurrencyConfig = (currencyCode: string): CurrencyConfig => {
  const config = currencyConfigs[currencyCode];
  if (!config) {
    throw new Error(`Unsupported currency: ${currencyCode}`);
  }
  return config;
};

/**
 * Validates a Money object.
 *
 * @param money - The object to validate
 * @throws Error if the object is not a valid Money object
 */
const validateMoney = (money: Money): void => {
  if (typeof money.amount !== 'number' || !Number.isInteger(money.amount)) {
    throw new Error('Amount must be an integer');
  }
  if (typeof money.currency !== 'string' || money.currency.length !== 3) {
    throw new Error('Currency must be a 3-letter ISO 4217 code');
  }
  getCurrencyConfig(money.currency); // This will throw if the currency is not supported
};

/**
 * Creates a Money object from an amount in the smallest unit of the currency and currency code.
 *
 * @param amount - The amount in the smallest unit of the currency (e.g., cents for USD)
 * @param currency - The ISO 4217 currency code (e.g., 'USD')
 * @returns A Money object
 * @throws Error if the input is invalid
 */
const money = (amount: number, currency: string): Money => {
  const baseUnits = Math.round(amount); // Ensure we're working with integers
  const moneyObj: Money = {
    amount: baseUnits,
    currency: currency.toUpperCase(),
    add(other: Money): Money {
      validateMoney(this);
      validateMoney(other);
      if (this.currency !== other.currency) {
        throw new Error('Cannot add different currencies');
      }
      return money(this.amount + other.amount, this.currency);
    },
    subtract(other: Money): Money {
      validateMoney(this);
      validateMoney(other);
      if (this.currency !== other.currency) {
        throw new Error('Cannot subtract different currencies');
      }
      return money(this.amount - other.amount, this.currency);
    },
    multiply(multiplier: number): Money {
      validateMoney(this);
      if (isNaN(multiplier)) {
        throw new Error('Multiplier must be a valid number');
      }
      return money(Math.round(this.amount * multiplier), this.currency);
    },
    divide(divisor: number): Money {
      validateMoney(this);
      if (isNaN(divisor)) {
        throw new Error('Divisor must be a valid number');
      }
      if (divisor === 0) {
        throw new Error('Cannot divide by zero');
      }
      return money(Math.round(this.amount / divisor), this.currency);
    },
    format(locale = 'en-IN'): string {
      validateMoney(this);
      const config = getCurrencyConfig(this.currency);
      const amount = this.amount / Math.pow(10, config.decimalPlaces);
      return new Intl.NumberFormat(locale, {
        style: 'currency',
        currency: this.currency,
        minimumFractionDigits: config.decimalPlaces,
        maximumFractionDigits: config.decimalPlaces,
      }).format(amount);
    },
    toDecimal(): number {
      validateMoney(this);
      const config = getCurrencyConfig(this.currency);
      return this.amount / Math.pow(10, config.decimalPlaces);
    },
    toBaseUnits(): number {
      validateMoney(this);
      return this.amount;
    }
  };
  validateMoney(moneyObj);
  return moneyObj;
};

/**
 * Converts a Money object to a decimal number.
 *
 * @param money - Money object to convert
 * @returns Decimal representation of the money amount
 */
const toDecimal = (money: Money): number => {
  validateMoney(money);
  const config = getCurrencyConfig(money.currency);
  return money.amount / Math.pow(10, config.decimalPlaces);
};

export type { Money };
export { money, toDecimal };
