import { redirect } from "@remix-run/react";

export const loader = () => redirect("/dashboard");

const Page = () => <div>Loading...</div>;

export default Page;
