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
import { userService } from "~/services/user";
import { roleService } from "~/services/roles";

export const loader = async ({ request }: LoaderFunctionArgs) => {
  const { cookies } = await auth.protected(request);
  const users = await userService.all(cookies);
  const roles = await roleService.all(cookies);
  return {
    users,
    roles,
  };
};

const Page = () => {
  const { users, roles } = useLoaderData<typeof loader>();

  return (
    <Card>
      <CardHeader>
        <CardTitle>Users</CardTitle>
        <CardDescription>You can manage your users from here</CardDescription>
      </CardHeader>
      <CardContent className="flex flex-col gap-4">
        <div className="flex items-center">
          <Input
            disabled
            placeholder="Search..."
            className="mr-auto max-w-md"
          />
          <Create roles={roles} />
        </div>
        <div className="border rounded-md">
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Email address</TableHead>
                <TableHead>Name</TableHead>
                <TableHead>Role</TableHead>
                <TableHead className="text-right" />
              </TableRow>
            </TableHeader>
            <TableBody>
              {users.map((user) => (
                <TableRow key={user.user.id}>
                  <TableCell className="font-medium">
                    {user.user.email}
                  </TableCell>
                  <TableCell className="font-medium">
                    {user.user.name || "N/A"}
                  </TableCell>
                  <TableCell className="font-medium">
                    {user.role.name[0].toUpperCase() + user.role.name.slice(1)}
                  </TableCell>
                  <TableCell className="text-right">
                    <Options roles={roles} user={user.user} />
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
