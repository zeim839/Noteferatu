import * as React from "react"
import { useExplorerContext } from "../context"
import { FileEntry } from "@/lib/helsync"
import { Entry } from "../entry"
import { ChevronDownIcon, ChevronRightIcon } from "lucide-react"

// Displays a list of tags sorted in a particular order when the
// explorer view is set to 'tags'.
function TagsView() {
  const explorer = useExplorerContext()
  const [expandedTags, setExpandedTags] = React.useState<Set<string>>(new Set())
  const [expandedFolders, setExpandedFolders] = React.useState<Set<string>>(new Set())

  const toggleTag = (tagName: string) => {
    setExpandedTags(prev => {
      const newSet = new Set(prev)
      if (newSet.has(tagName)) {
        newSet.delete(tagName)
      } else {
        newSet.add(tagName)
      }
      return newSet
    })
  }

  // Comparison function used to implement folder child sorting.
  const compareFn = (a: FileEntry, b: FileEntry): number => {
    const [valueA, valueB] = [a[explorer.sortFileKey()], b[explorer.sortFileKey()]]
    if (valueA < valueB) {
      return explorer.sortFileAsc() ? -1 : 1
    }
    if (valueA > valueB) {
      return explorer.sortFileAsc() ? 1 : -1
    }
    return 0
  }

  const sortedTags = [...explorer.tags()].sort((a, b) => {
    const asc = explorer.sortFileAsc()
    if (asc) {
      return a.name.localeCompare(b.name)
    }
    return b.name.localeCompare(a.name)
  })

  return (
    <div
      data-is-view-tags={explorer.view() === "tags"}
      className="w-full flex flex-col px-1 pt-1 flex-1 overflow-auto scrollbar-hide relative data-[is-view-tags=false]:hidden"
    >
      {sortedTags.map((tag) => {
        const isExpanded = expandedTags.has(tag.name)
        const hasChildren = tag.files && tag.files.length > 0
        return (
          <React.Fragment key={tag.name}>
            <div
              className="relative grid grid-cols-[20px_auto_20px] items-center font-light text-sm hover:bg-[#DCE0E8] hover:rounded-sm gap-2 h-[32px]"
              onClick={() => {
                if (hasChildren) {
                  toggleTag(tag.name)
                }
              }}
            >
              <div
                className="rounded-full h-[13px] w-[13px] ml-1"
                style={{ backgroundColor : tag.color}}
              />
              <p className="max-h-[17px] text-nowrap text-ellipsis overflow-x-hidden overflow-y-hidden">
                {tag.name}
              </p>
              {hasChildren ? (
                isExpanded ?
                  (<ChevronDownIcon className="size-4" strokeWidth={1.6} />) :
                  (<ChevronRightIcon className="size-4" strokeWidth={1.6} />)
              ) : null}
            </div>
            {isExpanded && hasChildren && (
              <>
                {[...tag.files].sort(compareFn).map((child, i) => (
                  <Entry
                    key={child.id}
                    file={child}
                    depth={1}
                    expandedFolders={expandedFolders}
                    setExpandedFolders={setExpandedFolders}
                    isLast={i === tag.files.length - 1}
                    sortFileKey={explorer.sortFileKey}
                    sortFileAsc={explorer.sortFileAsc}
                  />
                ))}
              </>
            )}
          </React.Fragment>
        )
      })}
    </div>
  )
}

export { TagsView }
