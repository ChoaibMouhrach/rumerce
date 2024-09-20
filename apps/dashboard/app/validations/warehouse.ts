import { z } from "zod";

export const warehouseSchema = z.object({
  id: z.string().uuid(),
  name: z.string().min(1),
  created_at: z.string().transform(Date),
});

export type Warehouse = z.infer<typeof warehouseSchema>;

export const storeWarehouseSchema = z.object({
  name: z.string().min(1),
});

export type StoreWarehousePayload = z.infer<typeof storeWarehouseSchema>;
