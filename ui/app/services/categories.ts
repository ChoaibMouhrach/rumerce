import { env } from "~/env";
import { api } from "./auth";
import { categorySchema, StoreCategoryPayload } from "~/validations/category";
import { z } from "zod";

class Category {
  public async all(cookie?: string) {
    const url = new URL(env.VITE_API_URL);
    url.pathname = "/categories";

    const response = await api(url, {
      headers: cookie ? { cookie } : undefined,
    });

    const data = await response.json();

    return z.array(categorySchema).parse(data);
  }

  public async show(id: string, cookie?: string) {
    const url = new URL(env.VITE_API_URL);
    url.pathname = `/categories/${id}`;

    const response = await api(url, {
      headers: cookie ? { cookie } : undefined,
    });

    const data = await response.json();

    return categorySchema.parse(data);
  }

  public store(payload: StoreCategoryPayload, cookie?: string) {
    const url = new URL(env.VITE_API_URL);
    url.pathname = "/categories";

    return api(url, {
      method: "POST",
      headers: cookie ? { cookie } : undefined,
      body: JSON.stringify(payload),
    });
  }

  public update(id: string, payload: StoreCategoryPayload, cookie?: string) {
    const url = new URL(env.VITE_API_URL);
    url.pathname = `/categories/${id}`;

    return api(url, {
      method: "PATCH",
      headers: cookie ? { cookie } : undefined,
      body: JSON.stringify(payload),
    });
  }

  public destroy(id: string, cookie?: string) {
    const url = new URL(env.VITE_API_URL);
    url.pathname = `/categories/${id}`;

    return api(url, {
      method: "DELETE",
      headers: cookie ? { cookie } : undefined,
    });
  }
}

export const categoryService = new Category();
