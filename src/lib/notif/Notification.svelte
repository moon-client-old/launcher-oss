<script lang="ts">
    import markdownIt from 'markdown-it/';
    import {
        callbacks,
        registerCallback,
        type Notification,
    } from './NotifHandler';
    import {onMount} from 'svelte';
    import {Icon} from "@steeze-ui/svelte-icon";
    import {ExclamationCircle} from "@steeze-ui/heroicons";

    const md = markdownIt({
        breaks: true,
        linkify: true,
        typographer: true,
        html: false,
    });
    export let notification: Notification;
    let firstFramePassed = false;
    onMount(() => {
        setTimeout(() => {
            firstFramePassed = true;
        }, 0);
    });
    $: actions = [
        ...(notification.actions ?? []),
        ...(notification.dismissable
            ? [{name: 'Dismiss', callback: 'dismiss-notif'}]
            : []),
    ];
    const generateFallbackCallback = (callback: string) => () => {
        console.warn(`Callback ${callback} not found`);
        return true;
    };
</script>

<div
        class="notif {notification.type} flex-col"
>
    <div class="p-3">
        <div class="flex flex-row">
            <Icon class="w-9 mr-3" src={ ExclamationCircle }></Icon>
            <div>
                {#if notification.title}
                    <div class="font-bold">{notification.title}</div>
                {/if}
                <div class="text-xs pt-1">
                    {#if notification.message}
                        {@html md.render(notification.message)}
                    {/if}
                </div>
            </div>
        </div>
        <div class="pt-1" style="padding-left: 38px">
            {#if actions.length > 0}
                {#each actions as action}
                    <button
                            class="bg-slate-900 mr-2 rounded-md py-1 px-1.5 text-xs text-neutral-300"
                            on:click={() => {
    					if (action.callback) {
    						const result = (
    							callbacks[action.callback] ??
    							generateFallbackCallback(action.callback)
    						)(action, notification);
    						if (result) notification.dismiss();
    					}
    				}}
                    >
                        {action.name}
                    </button>
                {/each}
            {/if}
        </div>
    </div>
</div>

<style lang="scss">
  @use 'sass:color';

  $errorAccent: #ff7777ff;
  $infoAccent: #77c9ff;
  $successAccent: #77ff77ff;
  $warningAccent: #ffff77ff;
  $baseColor: #1e293b;
  .notif {
    display: flex;
    background: var(--background, #fff2);
    backdrop-filter: blur(10px);
    border-radius: 0.5rem;
    width: 100%;
    border: 1px solid var(--accent);
    position: relative;
    z-index: 2;

    &.error {
      --accent: #{$errorAccent};
      --background: #{color.adjust($errorAccent, $blackness: +100%, $alpha: -0.7)};
    }

    &.info {
      --accent: #{$infoAccent};
      --background: #{color.adjust($infoAccent, $blackness: +100%, $alpha: -0.7)};
    }

    &.success {
      --accent: #{$successAccent};
      --background: #{color.adjust($successAccent, $blackness: +100%, $alpha: -0.7)};
    }

    &.warning {
      --accent: #{$warningAccent};
      --background: #{color.adjust($warningAccent, $blackness: +100%, $alpha: -0.7)};
    }

    &::before,
    &::after {
      content: '';
      display: block;
      height: 100%;
      width: 100%;
      position: absolute;
      top: 0;
      left: 0;
      background: var(--accent);
      opacity: 0.2;
      filter: blur(25px);
      pointer-events: none;
    }

    &::before {
      z-index: 1;
    }

    &::after {
      z-index: 3;
    }
  }
</style>
