import { env } from "~/env";
import { imageSchema } from "~/validations/image";

class Image {
  public async upload(file: File) {
    const url = new URL(env.VITE_API_URL);
    url.pathname = "/images";

    const formData = new FormData();
    formData.append(file.name, file);

    const response = await fetch(url, {
      credentials: "include",
      method: "POST",
      body: formData,
    });

    const data = await response.json();
    return imageSchema.parse(data);
  }
}

export const imageService = new Image();
