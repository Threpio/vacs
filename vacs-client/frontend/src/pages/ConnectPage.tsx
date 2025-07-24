import Button from "../components/ui/Button.tsx";

function ConnectPage() {
    return (
        <div className="h-full w-full flex justify-center items-center p-4">
            <Button
                color="green"
                className="w-auto px-10 py-3 text-xl"
            >
                Connect
            </Button>
        </div>
    );
}

export default ConnectPage;