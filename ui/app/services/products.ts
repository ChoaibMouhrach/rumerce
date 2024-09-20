import { z } from "zod";
import { env } from "~/env";
import {
  StoreProductPayload,
  collectionSchema,
  keySchema,
  productSchema,
  valueSchema,
  variantSchema,
} from "~/validations/product";
import { api } from "./auth";
import { categorySchema } from "~/validations/category";
import { unitSchema } from "~/validations/unit";
import { imageSchema } from "~/validations/image";

class ProductService {
  public async all(cookie?: string) {
    const url = new URL(env.VITE_API_URL);
    url.pathname = "/products";

    const response = await api(url, {
      headers: cookie ? { cookie } : undefined,
    });

    const data = await response.json();

    return z
      .array(
        z.object({
          product: productSchema,
          category: categorySchema,
          unit: unitSchema,
          images: z.array(imageSchema).min(1),
          variants: z
            .array(
              z.object({
                variant: variantSchema,
                collections: z
                  .array(
                    z.object({
                      collection: collectionSchema,
                      key: keySchema,
                      value: valueSchema,
                    })
                  )
                  .min(1),
              })
            )
            .min(1),
        })
      )
      .parse(data);
  }

  public async show(id: string, cookie?: string) {
    const url = new URL(env.VITE_API_URL);
    url.pathname = `/products/${id}`;

    const response = await api(url, {
      headers: cookie ? { cookie } : undefined,
    });

    const data = await response.json();
    return productSchema.parse(data);
  }

  public store(
    payload: Omit<StoreProductPayload, "images"> & { images: string[] },
    cookie?: string
  ) {
    const url = new URL(env.VITE_API_URL);
    url.pathname = "/products";

    return api(url, {
      method: "POST",
      headers: cookie ? { cookie } : undefined,
      body: JSON.stringify(payload),
    });
  }

  public update(
    id: string,
    payload: Omit<StoreProductPayload, "images"> & { images: string[] },
    cookie?: string
  ) {
    const url = new URL(env.VITE_API_URL);
    url.pathname = `/products/${id}`;

    return api(url, {
      method: "PATCH",
      headers: cookie ? { cookie } : undefined,
      body: JSON.stringify(payload),
    });
  }

  public destroy(id: string, cookie?: string) {
    const url = new URL(env.VITE_API_URL);
    url.pathname = `/products/${id}`;

    return api(url, {
      method: "DELETE",
      headers: cookie ? { cookie } : undefined,
    });
  }
}

export const productService = new ProductService();
