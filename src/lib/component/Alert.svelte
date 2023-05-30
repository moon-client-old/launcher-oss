<script lang="ts">
    import { Icon } from "@steeze-ui/svelte-icon";
    import { CheckCircle, ExclamationCircle } from "@steeze-ui/heroicons";
    
    type AlertType = "SUCCESS" | "WARNING" | "ERROR";

    export let visible: boolean;
    export let text: string;
    export let type: AlertType = "ERROR";

    const type_translation = {
        "SUCCESS": {
            color: "bg-green-600/[0.25]",
            border_color: "border-green-500",
            icon_color: "text-green-400",
            icon: CheckCircle
        },
        "WARNING": {
            color: "bg-amber-600/[0.25]",
            border_color: "border-amber-500",
            icon_color: "text-amber-300",
            icon: ExclamationCircle
        },
        "ERROR": {
            color: "bg-red-600/[0.25]",
            border_color: "border-red-500",
            icon_color: "text-red-300",
            icon: ExclamationCircle
        },
    }
</script>

{#if visible}
    <div class="flex flex-row items-center space-x-1 shrink-0 { type_translation[type].color } border { type_translation[type].border_color } rounded-lg my-1 px-2 py-1 ">
        <Icon class="{ type_translation[type].icon_color } w-6 min-w-4" src={ type_translation[type].icon } theme="outline" />
        <div>
            <p class="break-words text-xs">{ text }</p> 
        </div>
    </div>
{/if}