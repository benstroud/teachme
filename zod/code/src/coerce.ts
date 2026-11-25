import { z } from "zod";

// Coercion converts before checking. This is convenient for query strings and forms.
export const Quantity = z.coerce.number().int().positive();

// The trap: Number("") is 0, so an empty string coerces to a valid count of 0.
// Coercion turns "not provided" into a valid-looking value unless you check for it.
export const Count = z.coerce.number().int().nonnegative();

export const SafeQuantity = z.preprocess(
  (v) => (v === "" ? undefined : v),
  z.coerce.number().int().positive(),
);
