import { Controller, Post, UploadedFile, UseInterceptors } from '@nestjs/common';
import { FileInterceptor } from '@nestjs/platform-express';
import * as wasm from '../../pkg/img2webp';

@Controller('image')
export class ImageController {
  @Post('convert')
  @UseInterceptors(FileInterceptor('file'))
  async convertImage(@UploadedFile() file: Express.Multer.File) {
    const uint8Array = new Uint8Array(file.buffer);
    const result = wasm.process_image_to_webp(uint8Array);
    return { size: result.length, data: Buffer.from(result).toString('base64') };
  }
}