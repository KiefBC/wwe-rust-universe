<script>
    import "../app.css";

    let isBannerVisible = true;
    let isProfileDropdownVisible = false;

</script>

<!--
TODO: Add Login State
TODO: Hide Certain Nav Items if not logged in and add a login button/register button
TODO: <DEV> Add dropping of tables when closing the app
-->

<!-- VERSION BANNER -->
{#if isBannerVisible}
    <div>
        <div class="flex items-center gap-x-6 bg-gray-900 px-6 py-2.5 sm:px-3.5 sm:before:flex-1">
            <p class="text-sm leading-6 text-white">
                <a href="https://github.com/KiefBC/wwe-rust-universe" target="_blank">
                    <strong class="font-semibold">Version 0.0.5</strong>
                    <svg viewBox="0 0 2 2" class="mx-2 inline h-0.5 w-0.5 fill-current" aria-hidden="true">
                        <circle cx="1" cy="1" r="1"/>
                    </svg>
                    This is a test <span aria-hidden="true">&rarr;</span>
                </a>
            </p>
            <div class="flex flex-1 justify-end">
                <button type="button" class="-m-3 p-3 focus-visible:outline-offset-[-4px]"
                        on:click={() => isBannerVisible = false}>
                    <span class="sr-only">Dismiss</span>
                    <svg class="h-5 w-5 text-white" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                        <path d="M6.28 5.22a.75.75 0 00-1.06 1.06L8.94 10l-3.72 3.72a.75.75 0 101.06 1.06L10 11.06l3.72 3.72a.75.75 0 101.06-1.06L11.06 10l3.72-3.72a.75.75 0 00-1.06-1.06L10 8.94 6.28 5.22z"/>
                    </svg>
                </button>
            </div>
        </div>
    </div>
{/if}

<!-- MAIN CONTENT CONTAINER -->
<div class="min-h-full">
    <!-- NAVBAR -->
    <div class="mb-12">
        <nav class="">
            <div class="mx-auto max-w-7xl sm:px-6 lg:px-8">
                <div class="border-b border-gray-700">
                    <div class="flex h-16 items-center justify-between px-4 sm:px-0">
                        <div class="flex items-center">
                            <div class="flex-shrink-0">
                                <img class="h-8 w-8" src="logo.png" alt="Your Company">
                            </div>
                            <div class="hidden md:block">
                                <div class="ml-10 flex items-baseline space-x-4">
                                    <!-- Current: "bg-gray-900 text-white", Default: "text-gray-300 hover:bg-gray-700 hover:text-white" -->
                                    <a href="/" class="bg-gray-900 text-white rounded-md px-3 py-2 text-sm font-medium"
                                       aria-current="page">Dashboard</a>
                                    <a href="/"
                                       class="text-gray-300 hover:bg-gray-700 hover:text-white rounded-md px-3 py-2 text-sm font-medium">Superstars</a>
                                    <a href="/"
                                       class="text-gray-300 hover:bg-gray-700 hover:text-white rounded-md px-3 py-2 text-sm font-medium">Shows</a>
                                    <a href="/"
                                       class="text-gray-300 hover:bg-gray-700 hover:text-white rounded-md px-3 py-2 text-sm font-medium">Champions</a>
                                    <a href="/"
                                       class="text-gray-300 hover:bg-gray-700 hover:text-white rounded-md px-3 py-2 text-sm font-medium">Stats
                                        & Rankings</a>
                                </div>
                            </div>
                        </div>
                        <div class="hidden md:block">
                            <div class="ml-4 flex items-center md:ml-6">
                                <button type="button"
                                        class="rounded-full  p-1 text-gray-400 hover:text-white focus:outline-none focus:ring-2 focus:ring-white focus:ring-offset-2 focus:ring-offset-gray-800"
                                        disabled>
                                    <span class="sr-only">View notifications</span>
                                    <svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
                                         stroke="currentColor" aria-hidden="true">
                                        <path stroke-linecap="round" stroke-linejoin="round"
                                              d="M14.857 17.082a23.848 23.848 0 005.454-1.31A8.967 8.967 0 0118 9.75v-.7V9A6 6 0 006 9v.75a8.967 8.967 0 01-2.312 6.022c1.733.64 3.56 1.085 5.455 1.31m5.714 0a24.255 24.255 0 01-5.714 0m5.714 0a3 3 0 11-5.714 0"/>
                                    </svg>
                                </button>

                                <!-- Profile dropdown -->
                                <div class="relative ml-3">
                                    <div>
                                        <button type="button"
                                                class="flex max-w-xs items-center rounded-full  text-sm focus:outline-none focus:ring-2 focus:ring-white focus:ring-offset-2 focus:ring-offset-gray-800"
                                                id="user-menu-button" aria-expanded="false" aria-haspopup="true"
                                                on:click|preventDefault={() => isProfileDropdownVisible = !isProfileDropdownVisible}>
                                            <span class="sr-only">Open user menu</span>
                                            <img class="h-8 w-8 rounded-full"
                                                 src="https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80"
                                                 alt="">
                                        </button>
                                    </div>

                                    <!--
                                      Dropdown menu, show/hide based on menu state.

                                      Entering: "transition ease-out duration-100"
                                        From: "transform opacity-0 scale-95"
                                        To: "transform opacity-100 scale-100"
                                      Leaving: "transition ease-in duration-75"
                                        From: "transform opacity-100 scale-100"
                                        To: "transform opacity-0 scale-95"
                                    -->
                                    {#if isProfileDropdownVisible}
                                        <div class="absolute right-0 z-10 mt-2 w-48 origin-top-right rounded-md bg-white py-1 shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none"
                                             role="menu" aria-orientation="vertical" aria-labelledby="user-menu-button"
                                             tabindex="-1">
                                            <!-- Active: "bg-gray-100", Not Active: "" -->
                                            <a href="/profile" class="block px-4 py-2 text-sm text-gray-700"
                                               role="menuitem"
                                               tabindex="-1" id="user-menu-item-0">Your Profile</a>
                                            <a href="/profile" class="block px-4 py-2 text-sm text-gray-700"
                                               role="menuitem"
                                               tabindex="-1" id="user-menu-item-1">Settings</a>
                                            <a href="/login" class="block px-4 py-2 text-sm text-gray-700"
                                               role="menuitem"
                                               tabindex="-1" id="user-menu-item-2">Sign out</a>
                                        </div>
                                    {/if}
                                </div>
                            </div>
                        </div>
                        <div class="-mr-2 flex md:hidden">
                            <!-- Mobile menu button -->
                            <button type="button"
                                    class="inline-flex items-center justify-center rounded-md  p-2 text-gray-400 hover:bg-gray-700 hover:text-white focus:outline-none focus:ring-2 focus:ring-white focus:ring-offset-2 focus:ring-offset-gray-800"
                                    aria-controls="mobile-menu" aria-expanded="false">
                                <span class="sr-only">Open main menu</span>
                                <!-- Menu open: "hidden", Menu closed: "block" -->
                                <svg class="block h-6 w-6" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
                                     stroke="currentColor" aria-hidden="true">
                                    <path stroke-linecap="round" stroke-linejoin="round"
                                          d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5"/>
                                </svg>
                                <!-- Menu open: "block", Menu closed: "hidden" -->
                                <svg class="hidden h-6 w-6" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
                                     stroke="currentColor" aria-hidden="true">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12"/>
                                </svg>
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </nav>
    </div>
    <!-- MAIN CONTENT -->
    <main class="flex-grow">
        <div class="mx-auto max-w-7xl px-4 pb-12 sm:px-6 lg:px-8">
            <!-- Slot for the page content to be rendered -->
            <slot/>
        </div>
    </main>
</div>