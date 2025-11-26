import axios from 'axios';
import Routes from '../../../common/routes';

export const executeGETRequest = async (url: string, params: any): Promise<any> => {
    let data = await axios.get(url, {
        params: params
    }).then(
        res => res.data
    ).catch((err) => {
        console.error(err)
    });
    return data;
}