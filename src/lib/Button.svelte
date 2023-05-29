<script lang="ts">
    type ButtonTheme = "BLUE" | "GREEN" | "YELLOW" | "RED";

    const color_translation = {
        "BLUE": [
            "bg-blue-600",
            "hover:bg-blue-500"
        ],
        "GREEN": [
            "bg-green-500",
            "hover:bg-green-400"
        ],
        "YELLOW": [
            "bg-amber-500",
            "hover:bg-amber-400"
        ],
        "RED": [
            "bg-red-700",
            "hover:bg-red-600"
        ]
    }

    import { createEventDispatcher } from "svelte";

    export let text: string = "Button";
    export let color: ButtonTheme = "BLUE";

    const dispatch = createEventDispatcher();
</script>

<button class="w-full { color_translation[color][0] } rounded-lg text-sm py-2.5 mt-2 { color_translation[color][1] } transition" on:click={ () => dispatch("click") }>
    { text }
</button>
