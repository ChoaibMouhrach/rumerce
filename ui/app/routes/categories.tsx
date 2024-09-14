import { LoaderFunction } from "@remix-run/node";
import { useLoaderData } from "@remix-run/react";
import { env } from "~/env";

export const loader: LoaderFunction = async ({ request }) => {
  const cookie = request.headers.get("cookie");

  const apiUrl = new URL(env.API_URL);
  apiUrl.pathname = "/categories";

  const response = await fetch(apiUrl, {
    method: "GET",
    headers: {
      cookie: cookie!,
    },
  });

  const categories = await response.json();

  return {
    categories,
  };
};

const Page = () => {
  const data = useLoaderData<typeof loader>();

  return <div>{JSON.stringify(data, null, 4)}</div>;
};

export default Page;
