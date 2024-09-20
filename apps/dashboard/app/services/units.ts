import { z } from "zod";
import { env } from "@/env";
import { StoreUnitPayload, unitSchema } from "@/validations/unit";
import { api } from "./auth";

class UnitService {
  public async all(cookie?: string) {
    const url = new URL(env.VITE_API_URL);
    url.pathname = "/units";

    const response = await api(url, {
      headers: cookie ? { cookie } : undefined,
    });

    const data = await response.json();

    return z.array(unitSchema).parse(data);
  }

  public async show(id: string, cookie?: string) {
    const url = new URL(env.VITE_API_URL);
    url.pathname = `/units/${id}`;

    const response = await api(url, {
      headers: cookie ? { cookie } : undefined,
    });

    const data = await response.json();
    return unitSchema.parse(data);
  }

  public store(payload: StoreUnitPayload, cookie?: string) {
    const url = new URL(env.VITE_API_URL);
    url.pathname = "/units";

    return api(url, {
      method: "POST",
      headers: cookie ? { cookie } : undefined,
      body: JSON.stringify(payload),
    });
  }

  public update(id: string, payload: StoreUnitPayload, cookie?: string) {
    const url = new URL(env.VITE_API_URL);
    url.pathname = `/units/${id}`;

    return api(url, {
      method: "PATCH",
      headers: cookie ? { cookie } : undefined,
      body: JSON.stringify(payload),
    });
  }

  public destroy(id: string, cookie?: string) {
    const url = new URL(env.VITE_API_URL);
    url.pathname = `/units/${id}`;

    return api(url, {
      method: "DELETE",
      headers: cookie ? { cookie } : undefined,
    });
  }
}

export const unitService = new UnitService();
