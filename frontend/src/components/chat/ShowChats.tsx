import { useEffect, useState } from "react";
import { useAuth } from "@/context/authContext";

type ChatMessage = {
    sender_id: string;
    content: string;
    timestamp: any;
};

type ShowChatsProps = {
    chatId: string | null;
};

const ShowChats = ({ chatId }: ShowChatsProps) => {
    const { user } = useAuth();
    const [messages, setMessages] = useState<ChatMessage[]>([]);

    const getChats = async () => {
        if (!chatId) return; // No chat selected yet

        try {
            const token = sessionStorage.getItem("token");
            if (!token) {
                console.error("No token found in sessionStorage");
                return;
            }

            const response = await fetch(
                `http://localhost:8000/api/messages/${chatId}`,
                {
                    method: "GET",
                    headers: {
                        "Content-Type": "application/json",
                        "Authorization": `Bearer ${token}`,
                    },
                }
            );

            if (!response.ok) {
                console.error("Failed to fetch chats", response.status);
                return;
            }

            const data = await response.json();
            setMessages(data);
        } catch (error) {
            console.error("Error fetching chats:", error);
        }
    };

    useEffect(() => {
        getChats();
    }, [chatId]);

    return (
        <div className="p-4 text-white">
            {!chatId && <p>Select a chat to view messages</p>}
            {chatId && messages.length === 0 && <p>No messages yet.</p>}
            {messages.map((msg, idx) => (
                <div
                    key={idx}
                    className={`p-2 my-1 rounded ${
                        msg.sender_id === user?.user_id ? "bg-blue-600" : "bg-gray-700"
                    }`}
                >
                    <p>{msg.content}</p>
                    <span className="text-xs text-gray-400">
                        {(() => {
                            if (msg.timestamp?.$date && typeof msg.timestamp.$date === "string") {
                                return new Date(msg.timestamp.$date).toLocaleString();
                            }
                            if (msg.timestamp?.$date?.$numberLong) {
                                return new Date(parseInt(msg.timestamp.$date.$numberLong)).toLocaleString();
                            }
                            if (typeof msg.timestamp === "string") {
                                return new Date(msg.timestamp).toLocaleString();
                            }
                            return "";
                        })()}
                    </span>
                </div>
            ))}
        </div>
    );
};

export default ShowChats;
