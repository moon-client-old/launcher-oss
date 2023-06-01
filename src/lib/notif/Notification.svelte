<script lang="ts">
    import markdownIt from 'markdown-it/';
    import {
        callbacks,
        registerCallback,
        type Notification,
        NotificationType,
    } from './NotificationHandler';
    import {onMount} from 'svelte';
    import {Icon} from '@steeze-ui/svelte-icon';
    import {
        CheckCircle,
        ExclamationCircle,
        ExclamationTriangle,
        InformationCircle,
        QuestionMarkCircle, XMark,
    } from '@steeze-ui/heroicons';
    import type {IconSource} from '@steeze-ui/heroicons/types';
    import Button from "$lib/component/Button.svelte";

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
    let isLinux = false;
    onMount(() => {
        isLinux = navigator.userAgent.includes('Linux');
    });
    let autoselectedIcon: IconSource = (() => {
        switch (notification.type) {
            case NotificationType.Error:
                return ExclamationCircle;
            case NotificationType.Info:
                return InformationCircle;
            case NotificationType.Success:
                return CheckCircle;
            case NotificationType.Warning:
                return ExclamationTriangle;
            default:
                return QuestionMarkCircle;
        }
    })();
</script>

<div class="notif {notification.type} flex-col">
    <div class="p-3">
        <div class="flex flex-row">
            <Icon style="min-width: 26px; min-height: 26px; width: 26px; height: 26px"
                  class="mr-3"
                  src={notification.icon ?? autoselectedIcon}
            />
            <div class="float-right"
                    style={[notification.title, notification.message].filter(
					(v) => !!v
				).length <= 1
					? 'display:flex;align-items:center;justify-content:center;'
					: ''}
            >
                {#if notification.title}
                    <div class="font-bold">{notification.title}</div>
                {/if}
                {#if notification.message}
                    <div class="text-xs pt-1">
                        {#if notification.message}
                            {@html md.render(notification.message)}
                        {/if}
                    </div>
                {/if}
            </div>
        </div>
        {#if actions.length > 0}
            <div
                    class="pt-1"
                    style="padding-left: 38px;{isLinux
					? 'margin-left:0.7rem;'
					: ''}"
            >
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
            </div>
        {/if}
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
    border-radius: 0.6rem;
    width: 100%;
    border: 1px solid var(--accent);
    position: relative;
    z-index: 2;
    transition: 0.5s;

    &.error {
      --accent: #{$errorAccent};
      --background: #{color.adjust(
					$errorAccent,
					$blackness: +100%,
					$alpha: -0.7
				)};
    }

    &.info {
      --accent: #{$infoAccent};
      --background: #{color.adjust(
					$infoAccent,
					$blackness: +100%,
					$alpha: -0.7
				)};
    }

    &.success {
      --accent: #{$successAccent};
      --background: #{color.adjust(
					$successAccent,
					$blackness: +100%,
					$alpha: -0.7
				)};
    }

    &.warning {
      --accent: #{$warningAccent};
      --background: #{color.adjust(
					$warningAccent,
					$blackness: +100%,
					$alpha: -0.7
				)};
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
