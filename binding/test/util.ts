export const readEnv = async () => {
    const envText = await Bun.file(".bitcoin/.env.btc").text();
    const envMap = new Map(
        envText.split('\n')
            .filter(line => line.trim() && !line.startsWith('#'))
            .map(line => {
                const [key, value] = line.split('=').map(part => part.trim());
                return [key, value];
            })
    );
    return envMap;
}