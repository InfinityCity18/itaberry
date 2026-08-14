export const currentpage = $state({
    displays: true
})

export const fetched_files = $state({
    files: new DataTransfer().files
})

export const uploaded_files = $state({
    files: new DataTransfer().files
})