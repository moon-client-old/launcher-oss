// Notif State Handler
import { writable } from 'svelte/store';

/** Action Interface */
export interface IAction {
	/** Action Callback Name */
	callback?: string,
	/** Action Name */
	name: string,
	/** Metadata */
	metadata?: any,
}

/** Notification Interface */
type INotification = {
	/** Identification used for removing it */
	id?: number,
	/** Notification title */
	title: string | null,
	/** Notification message */
	message: string,
	/** Notification type */
	type: 'success' | 'error' | 'warning' | 'info',
	/** Notification duration */
	duration: number,
	/** Notification timestamp */
	timestamp: number,
	/** Notification expiry */
	expires_at: number,
	/** Notification Actions */
}

/** Notification Class */
export class Notification implements INotification {
	/** Notification ID */
	id: number = 0;
	/** Notification title */
	title: string | null = null;
	/** Notification message */
	message: string = '';
	/** Notification type */
	type: 'success' | 'error' | 'warning' | 'info' = 'info';
	/** Notification duration */
	#duration: number = 0;
	/** Notification duration */
	get duration(): number {
		return this.#duration;
	}
	/** Notification duration */
	set duration(duration: number) {
		this.#duration = duration;
		this.#expires_at = this.timestamp + this.duration;
	}
	/** Notification timestamp */
	#timestamp: number = 0;
	/** Notification timestamp */
	get timestamp(): number {
		return this.#timestamp;
	}
	/** Notification expiry */
	#expires_at: number = 0;
	/** Notification expiry */
	get expires_at(): number {
		return this.#expires_at;
	}
	/** Notification expiry */
	set expires_at(expires_at: number) {
		this.#expires_at = expires_at;
		this.#duration = this.expires_at - this.timestamp;
	}
	/** Dismissable */
	dismissable: boolean = true;
	/** Notification Actions */
	actions: IAction[] = [];

	/** Notification Class Constructor */
	constructor(_title: string | null, _message: string | null, type: 'success' | 'error' | 'warning' | 'info' | null = 'info', duration: number | null = 5000, dismissable: boolean = true, actions: IAction[] = []) {
		this.id = Math.floor(Math.random() * 1000000000);
		let title: string | null = _title;
		let message: string | null = _message;
		if (message === null) {
			message = _title;
			title = null;
		}
		if (message === null) throw new Error('Notification must have a message')
		this.title = title;
		this.message = message;
		this.type = type ?? 'info';
		this.duration = duration ?? 5000;
		this.#timestamp = Date.now();
		this.#expires_at = this.timestamp + this.duration;
		this.dismissable = dismissable;
		this.actions = actions;
	}

	/** Import from Notification */
	static import(notification: INotification): Notification {
		let notif = new Notification(notification.title, notification.message, notification.type, notification.duration);
		notif.id = notification.id ?? notif.id;
		notif.#timestamp = notification.timestamp;
		notif.#expires_at = notification.expires_at;
		return notif;
	}

	/** Validate INotification */
	static isValidExport(notification: INotification): boolean {
		if (typeof notification !== 'object') return false;
		if (typeof notification.id !== 'number') return false;
		if (typeof notification.title !== 'string' && notification.title !== null) return false;
		if (typeof notification.message !== 'string') return false;
		if (typeof notification.type !== 'string') return false;
		if (typeof notification.duration !== 'number') return false;
		if (typeof notification.timestamp !== 'number') return false;
		if (typeof notification.expires_at !== 'number') return false;
		return true;
		// ty gh copilot
	}

	/** Check if is expired */
	isExpired(): boolean {
		return Date.now() > this.expires_at;
	}

	/** Export */
	export(): string {
		return JSON.stringify({
			id: this.id,
			title: this.title,
			message: this.message,
			type: this.type,
			duration: this.duration,
			timestamp: this.timestamp,
			expires_at: this.expires_at,
		})
	}

	/** Get Expiration Progress */
	getExpirationProgress(): number {
		return (Date.now() - this.timestamp) / this.duration;
	}

	/** Destroy */
	destroy(): void {
		removeNotification(this.id);
	}
	/** Dismiss */
	dismiss(): void {
		this.destroy()
	}
}

export const notificationStore = writable<Notification[]>([])
export const removeNotification = (id: number) => {
	notificationStore.update((notifications) => {
		return notifications.filter((notif) => notif.id !== id);
	})
}
export const addNotification = (notification: Notification) => {
	// Push Notification
	notificationStore.update((notifications) => {
		notifications.push(notification);
		return notifications;
	})
	// Remove notification after duration
	setTimeout(() => {
		removeNotification(notification.id);
	}, Date.now() - notification.timestamp + notification.duration) // usually equals to notification.duration, unless added after export->reimport
}

// Save Notifications to Session Storage across reloads, if available
notificationStore.subscribe((notifications) => {
	if (typeof sessionStorage !== 'undefined') {
		sessionStorage.setItem('notifications', JSON.stringify(notifications.map((notification) => notification.export())));
	}
})

export const svelteMount = (() => {
	// Load Notifications from Session Storage, if available
	if (typeof sessionStorage !== 'undefined') {
		const notifications = sessionStorage.getItem('notifications');
		if (notifications !== null) {
			const parsed = JSON.parse(notifications);
			if (typeof parsed === 'object' && parsed.forEach) {
				const invalidNotifications = parsed.filter((notif: INotification) => !Notification.isValidExport(notif)).length
				if (invalidNotifications > 0) {
					console.warn(invalidNotifications + ' invalid notification objects in session storage');
				} else {
					parsed.forEach((notif: INotification) => {
						addNotification(Notification.import(notif));
					})
				}
			} else {
				console.warn('Invalid notifications object in session storage');
			}
		}
	}
})

/**
 * Notification Action
 * @param {IAction} action Action
 * @param {INotification} notification Notification
 * @returns {boolean | void} Return true to dismiss
 */
export type Callback = (action: IAction, notification: INotification) => boolean | void
export let callbacks: { [key: string]: Callback } = {}
export const registerCallback = (callback: string, callbackFunction: Callback) => {
	callbacks[callback] = callbackFunction;
	return callback;
}