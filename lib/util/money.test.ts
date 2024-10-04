import { money, toDecimal, Money } from "./money";

describe('money utility', () => {
  describe('money function', () => {
    it('should create a valid Money object', () => {
      const m = money(10000, 'USD');
      expect(m.amount).toBe(10000);
      expect(m.currency).toBe('USD');
    });

    it('should round the input amount to the nearest integer', () => {
      const m = money(100.6, 'USD');
      expect(m.amount).toBe(101);
    });

    it('should throw an error for invalid currency', () => {
      expect(() => money(100, 'INVALID')).toThrow('Currency must be a 3-letter ISO 4217 code');
    });
  });

  describe('Money object methods', () => {
    let usd100: Money, usd200: Money, eur100: Money;

    beforeEach(() => {
      usd100 = money(10000, 'USD');
      usd200 = money(20000, 'USD');
      eur100 = money(10000, 'EUR');
    });

    describe('add', () => {
      it('should correctly add two Money objects of the same currency', () => {
        const result = usd100.add(usd200);
        expect(result.amount).toBe(30000);
        expect(result.currency).toBe('USD');
      });

      it('should throw an error when adding different currencies', () => {
        expect(() => usd100.add(eur100)).toThrow('Cannot add different currencies');
      });
    });

    describe('subtract', () => {
      it('should correctly subtract two Money objects of the same currency', () => {
        const result = usd200.subtract(usd100);
        expect(result.amount).toBe(10000);
        expect(result.currency).toBe('USD');
      });

      it('should throw an error when subtracting different currencies', () => {
        expect(() => usd100.subtract(eur100)).toThrow('Cannot subtract different currencies');
      });
    });

    describe('multiply', () => {
      it('should correctly multiply a Money object by a number', () => {
        const result = usd100.multiply(2.5);
        expect(result.amount).toBe(25000);
        expect(result.currency).toBe('USD');
      });

      it('should round the result to the nearest integer', () => {
        const result = usd100.multiply(1.4);
        expect(result.amount).toBe(14000);
      });

      it('should throw an error for invalid multiplier', () => {
        expect(() => usd100.multiply(NaN)).toThrow('Multiplier must be a valid number');
      });
    });

    describe('divide', () => {
      it('should correctly divide a Money object by a number', () => {
        const result = usd100.divide(2);
        expect(result.amount).toBe(5000);
        expect(result.currency).toBe('USD');
      });

      it('should round the result to the nearest integer', () => {
        const result = usd100.divide(3);
        expect(result.amount).toBe(3333);
      });

      it('should throw an error for invalid divisor', () => {
        expect(() => usd100.divide(NaN)).toThrow('Divisor must be a valid number');
        expect(() => usd100.divide(0)).toThrow('Cannot divide by zero');
      });
    });

    describe('format', () => {
      it('should correctly format USD', () => {
        expect(usd100.format()).toBe('$100.00');
      });

      it('should correctly format EUR', () => {
        expect(eur100.format()).toBe('€100.00');
      });

      it('should respect locale', () => {
        expect(usd100.format('en-DE')).toBe('US$100.00');
      });
    });

    describe('toDecimal', () => {
      it('should correctly convert to decimal', () => {
        expect(usd100.toDecimal()).toBe(100);
        expect(money(10050, 'USD').toDecimal()).toBe(100.5);
      });
    });

    describe('toBaseUnits', () => {
      it('should return the amount in base units', () => {
        expect(usd100.toBaseUnits()).toBe(10000);
      });
    });
  });

  describe('toDecimal function', () => {
    it('should correctly convert Money object to decimal', () => {
      const m = money(10050, 'USD');
      expect(toDecimal(m)).toBe(100.5);
    });
  });

  describe('calc gst rounding', () => {
    let usd69: Money;

    beforeEach(() => {
      usd69 = money(6900, 'USD');
    });

    it('should correctly round 69 * 0.025', () => {
      const result = usd69.multiply(0.025);
      expect(result.amount).toBe(173);
    });

    it('should correctly round 69, 2.5% + 2.5%', () => {
      const result = usd69.multiply(0.025).add(usd69.multiply(0.025));
      expect(result.amount).toBe(346);
    });

    it('should correctly round 69 + 2.5% + 2.5%', () => {
      const result = usd69.add(usd69.multiply(0.025)).add(usd69.multiply(0.025));
      expect(result.amount).toBe(7246);
    });

    it('should correctly round 69 * 0.05', () => {
      const result = usd69.multiply(0.05);
      expect(result.amount).toBe(345);
    });

    it('should correctly round 69 * 1.05', () => {
      const result = usd69.multiply(1.05);
      expect(result.amount).toBe(7245);
    });
  });

  describe('edge cases', () => {
    it('should handle very large amounts', () => {
      const largeAmount = Number.MAX_SAFE_INTEGER;
      const m = money(largeAmount, 'USD');
      expect(m.amount).toBe(largeAmount);
    });

    it('should handle very small amounts', () => {
      const smallAmount = 1;
      const m = money(smallAmount, 'USD');
      expect(m.toDecimal()).toBe(0.01);
    });

    it('should handle zero amounts', () => {
      const m = money(0, 'USD');
      expect(m.format()).toBe('$0.00');
    });

    it('should handle negative amounts', () => {
      const m = money(-10000, 'USD');
      expect(m.format()).toBe('-$100.00');
    });

    it('should handle very large negative amounts', () => {
      const largeNegativeAmount = Number.MIN_SAFE_INTEGER;
      const m = money(largeNegativeAmount, 'USD');
      expect(m.amount).toBe(largeNegativeAmount);
    });
  });
});
