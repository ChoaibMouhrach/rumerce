import { z } from "zod";

export const categorySchema = z.object({
  id: z.string().uuid(),
  name: z.string().min(1),
  created_at: z.string().transform(Date),
});

export type Category = z.infer<typeof categorySchema>;

export const storeCategorySchema = z.object({
  name: z.string().min(1),
});

export type StoreCategoryPayload = z.infer<typeof storeCategorySchema>;
