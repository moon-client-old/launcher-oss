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
            ? [{name: 'Dismiss', callback: 'dismiss-notification'}]
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
    let color: String = (() => {
        switch(notification.type) {
            case NotificationType.Error:
                return "#f87171";
            case NotificationType.Info:
                return "#eab308";
            case NotificationType.Success:
                return "#22c55e";
            case NotificationType.Warning:
                return "#fb923c";
            default:
                return "";
        }
    })();
</script>

<div class="notif {notification.type} flex-col bg-slate-700/[0.25]">
    <div class="p-3">
        <div class="flex flex-row">
            <Icon style="min-width: 23px; min-height: 23px; width: 23px; height: 23px; color: {color}"
                  class="mr-2 mt-0.5"
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
                    <div class="font-extrabold" style="color: {color}">{notification.title}</div>
                {/if}
                {#if notification.message}
                    <div class="text-xs text-gray-300 pt-0.5">
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
                    style="padding-left: 30.5px;{isLinux
					? 'margin-left:0.7rem;'
					: ''}"
            >
                {#each actions as action}
                    <button
                            class="bg-slate-900/[0.35] mr-2 rounded-md py-1 px-1.5 text-xs text-gray-400"
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
    backdrop-filter: blur(10px);
    border-radius: 0.6rem;
    width: 100%;
    border: 1px solid rgb(71, 85, 105);
    position: relative;
    z-index: 2;
    transition: 0.5s;
    min-width: 250px;

    &::before,
    &::after {
      content: '';
      display: block;
      height: 100%;
      width: 100%;
      position: absolute;
      top: 0;
      left: 0;
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
