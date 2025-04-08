import Link from "next/link";

export default function Landing() {
  return (
    <div className="m-10">
      <main className="flex flex-col items-start gap-10 font-[font-family:var(--font-geist-sans)]">
        <Link href={"/login"}>Login Page</Link>
        <Link href={"/create"}>Register Page</Link>
        <Link href={"/forgot-password"}>Forgot Password Page</Link>
        <Link href={"/reset-password"}>Reset Password Page</Link>
        <Link href={"/home"}>Home Page</Link>
        <Link href={"/search"}>Search Page</Link>
        <Link href={"/profile"}>Profile Page</Link>
        <Link href={"/project"}>Project Page</Link>
        <Link href={"/account"}>Account Page</Link>
        <Link href={"/settings"}>Settings Page</Link>
        <Link href={"/about-us"}>About us Page</Link>
        <Link href={"/privacy-policy"}>Privacy Policy Page</Link>
        <Link href={"/terms-and-conditions"}>Terms and Conditions Page</Link>
      </main>
    </div>
  );
}