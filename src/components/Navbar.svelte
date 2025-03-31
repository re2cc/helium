<script>
</script>

<nav
    class="top-0 left-0 right-0 z-10 bg-white/10 backdrop-blur-md h-16 flex-shrink-0"
>
    <div
        class="container mx-auto px-4 flex items-center justify-between h-full"
    >
        <div class="text-xl font-bold text-white">Logo</div>
        <ul class="flex space-x-6 text-white">
            <li><a href="/" class="hover:text-gray-300">Home</a></li>
            <li><a href="#" class="hover:text-gray-300">About</a></li>
            <li><a href="#" class="hover:text-gray-300">Services</a></li>
            <li><a href="#" class="hover:text-gray-300">Contact</a></li>
            <li><slot name="account" /></li>
        </ul>
    </div>
</nav>
