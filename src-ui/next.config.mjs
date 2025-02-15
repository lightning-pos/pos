export async function redirects() {
    return [
        {
            source: '/',
            destination: '/login',
            permanent: true,
        },
    ]
}
