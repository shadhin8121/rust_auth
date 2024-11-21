"use client";
import React from "react";

const Page = () => {
    // Function to handle logout
    const handleLogout = async () => {
        try {
            const response = await fetch("http://localhost:5000/logout", {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                credentials: "include", // To include cookies
            });

            if (response.ok) {
                console.log("Logout successful");
                // Perform any additional actions on successful logout (like redirecting)
            } else {
                console.error("Logout failed");
            }
        } catch (error) {
            console.error("Error during logout:", error);
        }
    };

    return (
        <div className="flex justify-center items-center min-h-screen bg-gray-100">
            <button
                onClick={handleLogout}
                className="bg-blue-500 text-white py-2 px-4 rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-400 transition duration-300"
            >
                Logout
            </button>
        </div>
    );
};

export default Page;
