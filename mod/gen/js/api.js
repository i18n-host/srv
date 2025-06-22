import {
  T3Encode,T5Encode,T6Encode,
  T4Decode,T1Decode,
  T0Encode, //CallLiEncode,
  T0Decode // BinLiDecode
} from './_.pb.js'

import hpc from '-/lib/hpc.js'

const [
  _set,
  _noArgs,
  _req
] = hpc(T0Encode, T0Decode), NULL = ()=>{}

export const set = _set;

export const captchaVerify = (id /* [u8] */,click_pos_li /* [u32] */)=>_req(1,T4Decode,T3Encode([id,click_pos_li])) /* :string */
export const captcha = _noArgs(2,T1Decode) /* id:Vec<u8>,img:Vec<u8>,tip:Vec<u8> */
export const userName = _noArgs(3,T4Decode) /* :string */
export const authSigninMail = (address /* str */,password /* str */)=>_req(4,NULL,T5Encode([address,password]))
export const authSignupMail = (address /* str */,password /* str */)=>_req(5,NULL,T5Encode([address,password]))
export const authTest = (timezone /* i8 */,dpi /* u8 */,w /* u16 */,h /* u16 */,os_ver /* str */,arch /* str */,model /* str */,cpu_num /* u32 */,gpu /* str */)=>_req(6,NULL,T6Encode([timezone,dpi,w,h,os_ver,arch,model,cpu_num,gpu]))
export const demoCaptcha = _noArgs(7,NULL)
export const demoManualCaptcha = _noArgs(8,NULL)