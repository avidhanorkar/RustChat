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
    const userId =
        typeof user?.user_id === "object" && user?.user_id !== null && "$oid" in user.user_id
            ? (user.user_id as { $oid: string }).$oid
            : (user?.user_id as string | undefined);
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
                `http://localhost:8000/api/messages/${userId}/${chatId}`,
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
            console.log("Fetched chats:", data);
            setMessages(data);
        } catch (error) {
            console.error("Error fetching chats:", error);
        }
    };

    useEffect(() => {
        getChats();
    }, [chatId]);

    return (
        <div className=" text-white w-[60vw] h-[90vh] flex flex-col justify-between">

            {/* Messages container */}
            <div className="p-4 flex-1 overflow-y-auto">
                {!chatId && <p>Select a chat to view messages</p>}
                {chatId && messages.length === 0 && <p>No messages yet.</p>}

                {messages.map((msg, idx) => {
                    const isOwnMessage =
                        typeof msg.sender_id === "object" && msg.sender_id !== null && "$oid" in msg.sender_id
                            ? (msg.sender_id as { $oid: string }).$oid === userId
                            : msg.sender_id === userId;

                    return (
                        <div
                            key={idx}
                            className={`flex ${isOwnMessage ? "justify-end" : "justify-start"} my-1`}
                        >
                            <div
                                className={`p-2 rounded ${isOwnMessage ? "bg-blue-600 text-white" : "bg-gray-700 text-white"
                                    } max-w-[70%]`}
                            >
                                <p>{msg.content}</p>
                                <span className="text-xs text-gray-400 block text-right">
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
                        </div>
                    );
                })}
            </div>

            {/* Input box */}
            <div className="h-[10vh] flex flex-row justify-center px-2 items-center border-t-2 border-gray-500 gap-2">
                <input
                    className="bg-gray-700 text-white px-4 py-2 rounded-lg flex-1"
                    type="text"
                    placeholder="Type your message here..."
                />
                <button className="bg-blue-500 text-white px-4 py-2 rounded-lg">Send</button>
            </div>
        </div>
    );

};

export default ShowChats;
