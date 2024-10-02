import BaseApi from "./BaseApi";

export default class UserApi extends BaseApi {
    private static instance: UserApi;

    private baseUrl = `${this.apiUrl}/users`;

    public static getInstance(): UserApi {
        if (!UserApi.instance) {
            UserApi.instance = new UserApi();
        }

        return UserApi.instance;
    }

    public async getAll() {
        return this.getRequest<any[]>(this.baseUrl);
    }

    public async create(body: any) {
        return this.postRequest<any>(this.baseUrl, {
            ...body,
        });
    }
}
