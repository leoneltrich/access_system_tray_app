// src/lib/utils/format.ts

export function formatRemainingTime(rawMinutes: string | number | null | undefined): string {
    if (!rawMinutes) return '';
    const totalMinutes = typeof rawMinutes === 'string' ? parseInt(rawMinutes, 10) : rawMinutes;
    if (isNaN(totalMinutes)) return '';

    const hours = Math.floor(totalMinutes / 60);
    const minutes = totalMinutes % 60;

    if (hours > 0) return `${hours}h ${minutes}m`;
    return `${minutes}m`;
}