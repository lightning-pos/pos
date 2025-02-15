<script>
    import { Button, Link } from "carbon-components-svelte";
</script>

<div class="grid grid-cols-12 gap-4">
    <div
        class="col-span-8 bg-[url('login.jpg')] bg-cover bg-center bg-no-repeat"
    ></div>
    <div class="col-span-4">
        <div class="flex h-screen items-center">
            <div class="p-4">
                <Link href="/dash/pos">
                    <Button class="text-red-500">Log In</Button>
                </Link>
            </div>
        </div>
    </div>
</div>
