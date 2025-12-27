// $lib/utils/errorMapper.ts or inside your store
export function mapBackendError(error: any): string {
    const errString = String(error).toLowerCase();

    // 1. Backend Unreachable / Network Errors
    if (
        errString.includes('connection refused') ||
        errString.includes('network error') ||
        errString.includes('failed to fetch') ||
        errString.includes('timeout')
    ) {
        return "Backend Offline";
    }

    // 2. Unauthorized (401)
    if (errString.includes('401') || errString.includes('unauthorized')) {
        return "Please Log In";
    }

    // 3. Forbidden (403)
    if (errString.includes('403') || errString.includes('forbidden')) {
        return "Access Denied";
    }

    // 4. Not Found (404)
    if (errString.includes('404') || errString.includes('not found')) {
        return "Server Deleted";
    }

    // 5. Generic / Unknown
    // truncate if too long to fit in a label
    return "Request Failed";
}