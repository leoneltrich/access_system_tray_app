export function mapBackendError(error: any): string {
    const code = error?.message || String(error);

    switch (code) {
        case 'ERR_AUTH':
            return "Please Log In";
        case 'ERR_FORBIDDEN':
            return "Access Denied";
        case 'ERR_NOT_FOUND':
            return "Server Not Found";
        case 'ERR_DUPLICATE':
            return "Already Exists";
        case 'ERR_NETWORK':
            return "Backend Offline";
        case 'ERR_OFFLINE':
            return "Backend Offline";
        case 'ERR_SERVER':
            return "Server Error";
        case 'ERR_UNKNOWN':
            return "Unknown Error";
        case 'ERR_RATE_LIMIT':
            return "Too Many Requests";
    }

    return "Request Failed";
}