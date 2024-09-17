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
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "~/components/ui/card";
import { Create } from "./create";
import { Options } from "./options";
import { Pagination } from "~/components/pagination";
import { Search } from "~/components/search";
import { unitService } from "~/services/units";

export const loader = async ({ request }: LoaderFunctionArgs) => {
  const { cookies } = await auth.protected(request);
  const units = await unitService.all(cookies);
  return {
    units,
  };
};

const Page = () => {
  const { units } = useLoaderData<typeof loader>();

  return (
    <Card>
      <CardHeader>
        <CardTitle>Units</CardTitle>
        <CardDescription>You can manage your units from here</CardDescription>
      </CardHeader>
      <CardContent className="flex flex-col gap-4">
        <div className="flex items-center">
          <Search />
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
              {units.map((unit) => (
                <TableRow key={unit.id}>
                  <TableCell className="font-medium">{unit.name}</TableCell>
                  <TableCell className="text-right">
                    <Options unit={unit} />
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
