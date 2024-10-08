<script lang="ts">
  import init, { convert_to_cidrs, convert_to_addrs } from '$lib/wasm/pkg/ipcount_frontend.js';
  import { onMount } from 'svelte';

  onMount(() => {
    init();
  });

  let input = '';
  let output = '';
  let error = '';

  const onClick = () => {
    let toCidrError = '';
    let toAddrError = '';

    error = '';
    const inputs = input.split("\n").map((line) => line.trim()).filter((line) => line);
    try {
      output = convert_to_cidrs(inputs).join("\n");
    } catch (e) {
      toCidrError = e as string;
    }

    if (toCidrError) {
      try {
        output = convert_to_addrs(inputs).join("\n");
      } catch (e) {
        toAddrError = e as string;
      }
    }

    if (toAddrError) {
      error = `Error: converting to both direction failed. To CIDRs: ${toCidrError} & To address: ${toAddrError}`;
    }
  };
</script>

<head>
  <link rel="preconnect" href="https://fonts.gstatic.com/" crossorigin="" />
  <link
    rel="stylesheet"
    as="style"
    href="https://fonts.googleapis.com/css2?display=swap&amp;family=Noto+Sans%3Awght%40400%3B500%3B700%3B900&amp;family=Space+Grotesk%3Awght%40400%3B500%3B700"
  />

  <title>IP address ↔ CIDR Converter</title>
  <link rel="icon" type="image/x-icon" href="data:image/x-icon;base64," />

  <script src="https://cdn.tailwindcss.com?plugins=forms,container-queries"></script>
</head>
<body>
  <div
    class="relative flex size-full min-h-screen flex-col bg-[#111a22] dark group/design-root overflow-x-hidden"
    style="font-family: 'Space Grotesk', 'Noto Sans', sans-serif;"
  >
    <div class="layout-container flex h-full grow flex-col">
      <div class="px-40 flex flex-1 justify-center py-5">
        <div
          class="layout-content-container flex-col w-[512px] max-w-[512px] py-5 max-w-[960px] flex-1"
        >
          <div class="flex flex-wrap justify-between gap-3 p-4">
            <p class="text-white tracking-light text-[32px] font-bold leading-tight min-w-72">
              IP address ↔ CIDR Converter
            </p>
          </div>
          <div class="flex flex-1 flex-wrap items-end gap-4 px-4 py-3">
            <label class="flex flex-col h-300 min-w-40 flex-1">
              <p class="text-white text-base font-medium leading-normal pb-2">Input</p>
              <textarea
                placeholder="IP address or CIDR here..."
                class="form-input flex h-full w-full min-w-0 flex-1 resize-none rounded-xl text-white focus:outline-0 focus:ring-0 border border-[#344d65] bg-[#1a2632] focus:border-[#344d65] min-h-36 placeholder:text-[#93adc8] p-[15px] text-base font-normal leading-normal"
                bind:value={input}
              />
            </label>
          </div>

          <div class="flex px-4 py-3">
            <button
              class="flex min-w-0 cursor-pointer items-center justify-center rounded-xl h-10 px-4 flex-1 bg-[#1980e6] text-white text-sm font-bold leading-normal tracking-[0.015em]"
              on:click={onClick}
            >
              <span class="truncate">Convert</span>
            </button>
          </div>

          {#if error}
            <div class="flex px-4 py-3">
              <p class="text-red-500 text-base font-medium leading-normal pb-2">{error}</p>
            </div>
          {/if}

          <div class="flex flex-1 flex-wrap items-end gap-4 px-4 py-3">
            <label class="flex flex-col h-300 min-w-40 flex-1">
              <p class="text-white text-base font-medium leading-normal pb-2">
                Output
              </p>
              <textarea
                placeholder="Output will appear here..."
                class="form-input flex h-full w-full min-w-0 flex-1 resize-none rounded-xl text-white focus:outline-0 focus:ring-0 border border-[#344d65] bg-[#1a2632] focus:border-[#344d65] min-h-36 placeholder:text-[#93adc8] p-[15px] text-base font-normal leading-normal"
                bind:value={output}
                readonly
              />
            </label>
          </div>

          <div class="flex px-4 py-3">
            <button
              class="flex min-w-[84px] cursor-pointer items-center justify-center rounded-xl h-10 px-4 flex-1 bg-[#243647] text-white text-sm font-bold leading-normal tracking-[0.015em]"
              on:click={() => navigator.clipboard.writeText(output)}
            >
              <span class="truncate">Copy</span>
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</body>
