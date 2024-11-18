import Link from "next/link";

export default function Home() {
    return (
        <nav className="bg-gray-800 text-white">
            <div className="container mx-auto flex items-center justify-between p-4">
                {/* Logo Section */}
                <div className="text-xl font-bold">
                    <Link href="/">
                        <span>MyWebsite</span>
                    </Link>
                </div>

                {/* Navigation Links */}
                <ul className="flex space-x-6">
                    <li>
                        <Link href="/register">
                            <span className="hover:text-blue-400 transition">
                                Register
                            </span>
                        </Link>
                    </li>
                    <li>
                        <Link href="/login">
                            <span className="hover:text-green-400 transition">
                                Login
                            </span>
                        </Link>
                    </li>
                    <li>
                        <Link href="/logout">
                            <span className="hover:text-red-400 transition">
                                Logout
                            </span>
                        </Link>
                    </li>
                </ul>
            </div>
        </nav>
    );
}
