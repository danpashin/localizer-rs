
#import <Foundation/Foundation.h>
#import <UIKit/UIKit.h>
#import <substrate.h>

#ifdef __cplusplus
extern "C" {
#endif

void init_localizer(const char *to_localize_file_path);
const char *translation_file_name_for_address(uintptr_t address);

#ifdef __cplusplus
}
#endif

__attribute__((always_inline)) static inline uintptr_t return_address() {
    return (uintptr_t)__builtin_extract_return_addr(__builtin_return_address(0));
}

static NSString *translate_for(uintptr_t address, NSString *fallback) {
    const char *file_name = translation_file_name_for_address(address);
    if (file_name) {
        CFStringRef table_name = CFStringCreateWithCStringNoCopy(kCFAllocatorDefault, file_name, kCFStringEncodingASCII, kCFAllocatorNull);
        CFStringRef key = (__bridge CFStringRef)fallback;
        CFStringRef localized = CFBundleCopyLocalizedString(CFBundleGetMainBundle(), key, key, table_name);
        return CFBridgingRelease(localized);
    }
    
    return fallback;
}

%hook UILabel
- (void)setText:(NSString *)text {
    %log;
    text = translate_for(return_address(), text);
    %orig;
}
%end

%hook UITextField
- (void)setText:(NSString *)text {
    %log;
    text = translate_for(return_address(), text);
    %orig;
}
%end

%hook UIButton
- (void)setTitle:(NSString *)title forState:(UIControlState)state {
    %log;
    title = translate_for(return_address(), title);
    %orig;
}
%end

%ctor {
    CFBundleRef bundle = CFBundleGetMainBundle();
    CFURLRef URL = CFBundleCopyResourceURL(bundle, CFSTR("to-localize"), CFSTR("txt"), NULL);
    if (!URL) {
        return;
    }
    
    CFStringRef path = CFURLCopyFileSystemPath(URL, kCFURLPOSIXPathStyle);
    const char *c_path = CFStringGetCStringPtr(path, kCFStringEncodingASCII);
    init_localizer(c_path);
    
    CFRelease(path);
    CFRelease(URL);
    
    %init;
}
