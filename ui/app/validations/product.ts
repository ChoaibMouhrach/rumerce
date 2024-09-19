import { z } from "zod";

export const valueSchema = z.object({
  id: z.string().uuid(),
  name: z.string().min(1),
  key_id: z.string().uuid(),
});

export const keySchema = z.object({
  id: z.string().uuid(),
  name: z.string().min(1),
  product_id: z.string().uuid(),
});

export const collectionSchema = z.object({
  id: z.string().uuid(),
  variant_id: z.string().uuid(),
  key_id: z.string().uuid(),
  value_id: z.string().uuid(),
});

export const variantSchema = z.object({
  id: z.string().uuid(),
  price: z.number().gte(1),
  product_id: z.string().uuid(),
});

export const productSchema = z.object({
  id: z.string().uuid(),
  name: z.string().min(1),
  description: z.union([z.null(), z.string().min(1)]),
  unit_id: z.string().uuid(),
  category_id: z.string().uuid(),
});

export const storeProductSchema = z.object({
  name: z.string().min(1),
  description: z.union([z.null(), z.string().min(1)]),
  unit_id: z.string().uuid(),
  category_id: z.string().uuid(),
  images: z.array(z.instanceof(File)).min(1),
  variants: z
    .array(
      z.object({
        price: z.number().gte(1),
        checked: z.boolean(),
        options: z
          .array(
            z.object({
              key: z.string().min(1),
              value: z.string().min(1),
            })
          )
          .min(1),
      })
    )
    .min(1),
});

export type StoreProductPayload = z.infer<typeof storeProductSchema>;
