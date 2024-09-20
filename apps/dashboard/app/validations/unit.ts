import { z } from "zod";

export const unitSchema = z.object({
  id: z.string().uuid(),
  name: z.string().min(1),
  created_at: z.string().transform(Date),
});

export type Unit = z.infer<typeof unitSchema>;

export const storeUnitSchema = z.object({
  name: z.string().min(1),
});

export type StoreUnitPayload = z.infer<typeof storeUnitSchema>;
