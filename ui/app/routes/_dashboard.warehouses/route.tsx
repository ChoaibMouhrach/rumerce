import { LoaderFunctionArgs } from "@remix-run/node";
import { useLoaderData } from "@remix-run/react";
import { auth } from "~/services/auth";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "~/components/ui/table";
import { Input } from "~/components/ui/input";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "~/components/ui/card";
import { Create } from "./create";
import { Options } from "./options";
import { Pagination } from "./pagination";
import { warehouseService } from "~/services/warehouse";

export const loader = async ({ request }: LoaderFunctionArgs) => {
  const { cookies } = await auth.protected(request);
  const warehouses = await warehouseService.all(cookies);
  return {
    warehouses,
  };
};

const Page = () => {
  const { warehouses } = useLoaderData<typeof loader>();

  return (
    <Card>
      <CardHeader>
        <CardTitle>Warehouses</CardTitle>
        <CardDescription>
          You can manage your warehouses from here
        </CardDescription>
      </CardHeader>
      <CardContent className="flex flex-col gap-4">
        <div className="flex items-center">
          <Input
            disabled
            placeholder="Search..."
            className="mr-auto max-w-md"
          />
          <Create />
        </div>
        <div className="border rounded-md">
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Name</TableHead>
                <TableHead className="text-right" />
              </TableRow>
            </TableHeader>
            <TableBody>
              {warehouses.map((warehouse) => (
                <TableRow key={warehouse.id}>
                  <TableCell className="font-medium">
                    {warehouse.name}
                  </TableCell>
                  <TableCell className="text-right">
                    <Options warehouse={warehouse} />
                  </TableCell>
                </TableRow>
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
