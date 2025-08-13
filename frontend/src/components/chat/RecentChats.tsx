import { useAuth } from "@/context/authContext"
import { useEffect, useState } from "react"

type Chat = {
    chat_id: unknown;
    chat_type: 'user' | 'room';
    name: string;
    last_message: string;
    timestamp: any;
};

type RecentChatsProps = {
    onSelectChat: (chatId: string) => void;
};

const RecentChats = ({ onSelectChat }: RecentChatsProps) => {
    const { user } = useAuth();
    const [recentChats, setRecentChats] = useState<Chat[]>([]);
    const userId =
        typeof user?.user_id === "object" && user?.user_id !== null && "$oid" in user.user_id
            ? (user.user_id as { $oid: string }).$oid
            : (user?.user_id as string | undefined);

    const getRecentChats = async () => {
        try {
            const response = await fetch(`http://localhost:8000/api/message/${userId}`, {
                method: "GET",
                headers: {
                    "Authorization": `Bearer ${sessionStorage.getItem("token")}`,
                },
            });

            if (!response.ok) {
                console.error("Failed to fetch recent chats", response.status);
                return;
            }

            const data = await response.json();
            console.log("Fetched recent chats:", data);

            setRecentChats(data);
        } catch (error) {
            console.error("Error fetching recent chats:", error);
            return;
        }
    }

    useEffect(() => {
        if (userId) {
            getRecentChats();
        }
    }, [])

    return (
        <div className="w-full flex flex-col gap-2 pt-5 items-center ">
            {recentChats.map((chat, index) => (
                <div
                    key={index}
                    className='flex flex-col items-center justify-between gap-[1px] border rounded-lg border-gray-500 py-2 cursor-pointer w-[38vw] px-10 hover:bg-[#111]'
                    onClick={() => {
                        const chatIdStr =
                            typeof chat.chat_id === "object" && chat.chat_id !== null && "$oid" in chat.chat_id
                                ? (chat.chat_id as { $oid: string }).$oid
                                : (chat.chat_id as string);
                        onSelectChat(chatIdStr);
                    }}
                >
                    <div className="w-full">
                        <p className='text-white text-lg text-start'>{chat.name}</p>
                    </div>
                    <div className="flex flex-row justify-between w-full">
                        <p className="text-gray-300">{chat.last_message}</p>
                        <p className='text-gray-500 text-sm'>
                            {(() => {
                                if (chat.timestamp?.$date && typeof chat.timestamp.$date === "string") {
                                    return new Date(chat.timestamp.$date).toLocaleString();
                                }
                                if (chat.timestamp?.$date?.$numberLong) {
                                    return new Date(parseInt(chat.timestamp.$date.$numberLong)).toLocaleString();
                                }
                                if (typeof chat.timestamp === "string") {
                                    return new Date(chat.timestamp).toLocaleString();
                                }
                                return "";
                            })()}
                        </p>
                    </div>
                </div>
            ))}
        </div>
    );
};

export default RecentChats;
