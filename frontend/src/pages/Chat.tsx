import { useEffect, useState } from 'react';
import Navbar from '@/components/chat/Navbar';
import { useAuth } from '@/context/authContext';
import RecentChats from '@/components/chat/RecentChats';
import ShowChats from '@/components/chat/ShowChats';

const Chat = () => {
    const { user } = useAuth();

    const [selectedChatId, setSelectedChatId] = useState<string | null>(null);

    const handleChatSelect = (chatId: string) => {
        setSelectedChatId(chatId);
    };

    // Type definition for the user data from backend
    type UserDetails = {
        id: string;
        name: string;
        email: string;
    };

    const [userDet, setUserDet] = useState<UserDetails>({
        id: "",
        name: "",
        email: "",
    });

    // Extract userId string safely
    const userId =
        typeof user?.user_id === "object" && user?.user_id !== null && "$oid" in user.user_id
            ? (user.user_id as { $oid: string }).$oid
            : (user?.user_id as string | undefined);

    const getUserDetails = async () => {
        try {
            const response = await fetch(`http://localhost:8000/api/user/getUser/${userId}`, {
                method: "GET",
                headers: {
                    "Content-Type": "application/json",
                    "Authorization": `Bearer ${sessionStorage.getItem("token")}`,
                },
            });

            if (!response.ok) {
                console.error("Failed to fetch user details", response.status);
                return;
            }

            const data = await response.json();

            // Map backend's _id to id if necessary
            setUserDet({
                id: data.id || data._id || "",
                name: data.name || "",
                email: data.email || "",
            });
        } catch (error) {
            console.error("Error fetching user details:", error);
        }
    };

    useEffect(() => {
        if (userId) {
            getUserDetails();
        }
    }, [userId]);

    if (!user) {
        return (
            <div className='text-white text-5xl font-serif font-bold w-screen h-screen flex items-center justify-center'>
                Please login
            </div>
        );
    }

    return (
        <div>
            <Navbar username={userDet.name || ''} />
            <div className='flex flex-row items-center justify-center w-screen h-screen'>
                <div className='w-2/5 h-screen border-r-2 border-gray-500'>
                    <div className='flex flex-col items-center justify-center h-[8vh]'>
                        <p className='text-white font-semibold text-xl'>Recent Chats</p>
                    </div>
                    <RecentChats onSelectChat={handleChatSelect} />
                </div>
                <div className='w-3/5 h-screen'>
                    <ShowChats chatId={selectedChatId} />
                </div>
            </div>
        </div>
    );
};

export default Chat;
