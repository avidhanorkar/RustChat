import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import Login from "@/components/auth/login";
import Register from "@/components/auth/register";
import { Card } from "@/components/ui/card"

const Auth = () => {
    return (
        <div className=' flex w-screen h-screen flex-col items-center justify-center'>
            {/* <Tabs defaultValue="account" className="w-[25vw] bg-[#101010] rounded-2xl py-5 px-3">
                <TabsList className="w-full flex justify-center ">
                    <TabsTrigger className="text-white w-[45%] focus-visible:outline-none focus-visible:ring-0 border-none" value="account">Register</TabsTrigger>
                    <TabsTrigger className="text-white w-[45%] focus-visible:outline-none focus-visible:ring-0 border-none" value="password">Login</TabsTrigger>
                </TabsList>
                <TabsContent value="account"><Register /></TabsContent>
                <TabsContent value="password"><Login /></TabsContent>
            </Tabs> */}

            <div className="flex w-full max-w-sm flex-col gap-6">
                <Tabs defaultValue="account">
                    <TabsList className="flex gap-2 w-full items-center justify-center ">
                        <TabsTrigger className="text-white datat-[state=active]:bg-gray-800 bg-black" value="password">Register</TabsTrigger>
                        <TabsTrigger className="text-white datat-[state=active]:bg-gray-800 bg-black" value="account">Login</TabsTrigger>
                    </TabsList>
                    <TabsContent value="account">
                        <Card className="bg-[#ededed]">
                            <Login />
                        </Card>
                    </TabsContent>
                    <TabsContent value="password">
                        <Card className="bg-[#ededed]">
                            <Register />
                        </Card>
                    </TabsContent>
                </Tabs>
            </div>
        </div>
    )
}

export default Auth