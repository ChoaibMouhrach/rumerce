import { env } from "@/env";
import { api } from "./auth";
import {
  warehouseSchema,
  StoreWarehousePayload,
} from "@/validations/warehouse";
import { z } from "zod";

class Warehouse {
  public async all(cookie?: string) {
    const url = new URL(env.VITE_API_URL);
    url.pathname = "/warehouses";

    const response = await api(url, {
      headers: cookie ? { cookie } : undefined,
    });

    const data = await response.json();

    return z.array(warehouseSchema).parse(data);
  }

  public async show(id: string, cookie?: string) {
    const url = new URL(env.VITE_API_URL);
    url.pathname = `/warehouses/${id}`;

    const response = await api(url, {
      headers: cookie ? { cookie } : undefined,
    });

    const data = await response.json();

    return warehouseSchema.parse(data);
  }

  public store(payload: StoreWarehousePayload, cookie?: string) {
    const url = new URL(env.VITE_API_URL);
    url.pathname = "/warehouses";

    return api(url, {
      method: "POST",
      headers: cookie ? { cookie } : undefined,
      body: JSON.stringify(payload),
    });
  }

  public update(id: string, payload: StoreWarehousePayload, cookie?: string) {
    const url = new URL(env.VITE_API_URL);
    url.pathname = `/warehouses/${id}`;

    return api(url, {
      method: "PATCH",
      headers: cookie ? { cookie } : undefined,
      body: JSON.stringify(payload),
    });
  }

  public destroy(id: string, cookie?: string) {
    const url = new URL(env.VITE_API_URL);
    url.pathname = `/warehouses/${id}`;

    return api(url, {
      method: "DELETE",
      headers: cookie ? { cookie } : undefined,
    });
  }
}

export const warehouseService = new Warehouse();
