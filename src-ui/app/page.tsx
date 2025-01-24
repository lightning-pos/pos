import Link from "next/link";

export default function Home() {
  return (
    <div>
      <p className="text-xl">Hello, World!</p>
      <Link href={"/dash/pos"}>Dashboard</Link><br />
      <Link href={"/login"}>Login</Link><br />
      <Link href={"/dash/catalog/categories"}>Categories</Link>
    </div>
  );
}
