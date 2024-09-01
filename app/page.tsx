import Link from "next/link";

export default function Home() {
  return (
    <div>
      <p className="text-xl">Hello, World!</p>
      <Link href={"/dash/pos"}>Dashboard</Link>
      <Link href={"/login"}>Login</Link>
    </div>
  );
}
