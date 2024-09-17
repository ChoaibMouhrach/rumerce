import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "~/components/ui/card";
import { Pagination } from "~/components/pagination";
import { Search } from "~/components/search";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "~/components/ui/table";
import { Create } from "./create";
import { LoaderFunctionArgs } from "@remix-run/node";
import { productService } from "~/services/products";
import { auth } from "~/services/auth";
import { useLoaderData } from "@remix-run/react";
import { Options } from "./options";

export const loader = async ({ request }: LoaderFunctionArgs) => {
  const { cookies } = await auth.protected(request);
  const products = await productService.all(cookies);
  return {
    products,
  };
};

const Page = () => {
  const { products } = useLoaderData<typeof loader>();

  return (
    <Card>
      <CardHeader>
        <CardTitle>Products</CardTitle>
        <CardDescription>
          You can manage your products from here.
        </CardDescription>
      </CardHeader>
      <CardContent className="flex flex-col gap-4">
        <div className="flex items-center ">
          <Search />
          <Create />
        </div>
        <div className="border rounded-md">
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Image</TableHead>
                <TableHead>Name</TableHead>
                <TableHead>Category</TableHead>
                <TableHead>Unit</TableHead>
                <TableHead>Variants</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {products.map((product) => (
                <Options product={product} key={product.product.id}>
                  <TableRow>
                    <TableCell>
                      <div className="border rounded-md w-32 aspect-square bg-muted" />
                    </TableCell>
                    <TableCell className="font-medium">
                      {product.product.name}
                    </TableCell>
                    <TableCell>{product.category.name}</TableCell>
                    <TableCell>{product.unit.name}</TableCell>
                    <TableCell>{product.variants.length}</TableCell>
                  </TableRow>
                </Options>
              ))}
            </TableBody>
          </Table>
        </div>
        <Pagination />
      </CardContent>
    </Card>
  );
};

export default Page;
