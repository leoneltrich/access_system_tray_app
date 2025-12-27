export function mapBackendError(error: any): string {
    // 1. Get the message string
    const code = error?.message || String(error);

    // 2. Map standard codes
    switch (code) {
        case 'ERR_AUTH':        return "Please Log In";
        case 'ERR_FORBIDDEN':   return "Access Denied";
        case 'ERR_NOT_FOUND':   return "Server Not Found";
        case 'ERR_DUPLICATE':   return "Already Exists";
        case 'ERR_NETWORK':     return "Backend Offline";
        case 'ERR_OFFLINE':     return "Backend Offline";
        case 'ERR_SERVER':      return "Server Error";
        case 'ERR_UNKNOWN':     return "Unknown Error";
    }

    // 3. Fallback for unexpected errors
    return "Request Failed";
}