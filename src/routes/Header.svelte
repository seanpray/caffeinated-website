<script>
	import { page } from '$app/stores';
	import logo from '$lib/images/logo.png';
	import github from '$lib/images/github.svg';
</script>

    <header class="flex justify-center text-center lowercase opacity-100 mb-10 pb-10">
        <div class="fixed rounded-md z-50 pb-4">
            <nav class="bg-[#2d1902] rounded-sm bg-opacity-50 w-[100vw] pb-5">
                <ul>
                    <li aria-current={$page.url.pathname === '/' ? 'page' : undefined}>
                        <a class="text-white header-items m-3" href="/">about</a>
                    </li>
                    <!-- <li aria-current={$page.url.pathname.startsWith('/media') ? 'page' : undefined}> -->
                    <!--     <a class="text-white" href="/media">media</a> -->
                    <!-- </li> -->
                    <li aria-current={$page.url.pathname.startsWith('/blog') ? 'page' : undefined}>
                        <a class="text-white header-items m-3" href="/blog">blog</a>
                    </li>
                    <!-- <li aria-current={$page.url.pathname.startsWith('/robot') ? 'page' : undefined}> -->
                    <!--     <a class="text-white" href="/robot">Robot</a> -->
                    <!-- </li> -->
                    <li aria-current={$page.url.pathname.startsWith('/sponsors') ? 'page' : undefined}>
                        <a class="text-white header-items m-3" href="/sponsors">Sponsors</a>
                    </li>
                    <!-- <li aria-current={$page.url.pathname.startsWith('/member') ? 'page' : undefined}> -->
                    <!-- 	<a class="text-white" href="/member">Members</a> -->
                    <!-- </li> -->
                </ul>
            </nav>
        </div>
    </header>

<style>
	.header-items {
		//-webkit-text-stroke-width: 0.2px;
		//-webkit-text-stroke-color: black;
        font-size: 3vw;
	}
    @media (max-width: 800px) {
        .header-items {
            //-webkit-text-stroke-width: 0.2px;
            //-webkit-text-stroke-color: black;
            font-size: 1.3rem;
        }
    }
</style>
