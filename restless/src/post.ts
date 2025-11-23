export class Post {
    id : number;
    owner : string;
    contents : string;
    time : string;
    repost : object | null;
    total_likes : number;
}
